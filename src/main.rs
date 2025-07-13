#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{anyhow, Context, Result};
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;
use std::process::Command;
use tauri::Manager;

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Assets;

fn extract_assets(temp_path: &Path) -> Result<()> {
    for filename in Assets::iter() {
        let asset = Assets::get(&filename).unwrap();
        let dest_path = temp_path.join(filename.as_ref());
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory {}", parent.display()))?;
        }
        fs::write(&dest_path, asset.data)
            .context(format!("Failed to write asset {}", dest_path.display()))?;
        #[cfg(unix)]
        if filename == "node" || filename.ends_with("mmdc") {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o755))?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn render_mermaid_to_file(
    source_text: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let output_file_path = match tauri::api::dialog::FileDialogBuilder::new(&app_handle)
        .set_title("Save Diagram As")
        .add_filter("PNG Image", &["png"])
        .add_filter("SVG Image", &["svg"])
        .set_file_name("diagram.png")
        .save_file()
    {
        Some(path) => path,
        None => return Ok("Save cancelled by user.".to_string()),
    };

    if let Err(e) = render_mermaid_logic(&source_text, &output_file_path).await {
        return Err(e.to_string());
    }

    Ok(format!("Diagram saved to {}", output_file_path.display()))
}

async fn render_mermaid_logic(source_text: &str, output_file: &Path) -> Result<()> {
    let temp_dir = tempfile::Builder::new()
        .prefix("mermaid-render")
        .tempdir()
        .context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path();

    extract_assets(temp_path)?;

    let node_path = temp_path.join("node");
    let wrapper_script = temp_path.join("mmdc_wrapper.js");
    let input_path = temp_path.join("input.mmd");

    fs::write(&wrapper_script, include_str!("mmdc_wrapper.js"))?;
    fs::write(temp_path.join("package.json"), include_str!("package.json"))?;
    fs::write(&input_path, source_text)?;

    let output = Command::new(&node_path)
        .arg(&wrapper_script)
        .arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(output_file)
        .current_dir(&temp_path)
        .output()
        .context("Failed to execute mmdc process")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Mermaid CLI failed with status {}: {}",
            output.status,
            stderr
        ));
    }

    if !output_file.exists() {
        return Err(anyhow!(
            "Output file was not created at: {}",
            output_file.display()
        ));
    }

    #[cfg(target_os = "macos")]
    Command::new("open").arg(output_file).status()?;
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(output_file)
        .status()?;
    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(output_file).status()?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if Assets::get("node").is_none() || Assets::get("node_modules/.bin/mmdc").is_none() {
                tauri::api::dialog::MessageDialogBuilder::new(
                    "Asset Check Failed",
                    "Required resources not found. Please run 'setup_assets.sh' first to download Node.js and the Mermaid CLI.",
                )
                .kind(tauri::api::dialog::MessageDialogKind::Error)
                .show(|_| std::process::exit(1));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![render_mermaid_to_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
