use anyhow::{anyhow, Context, Result};
use arboard::Clipboard;
use lazy_static::lazy_static;
use rfd::FileDialog;
use rust_embed::RustEmbed;
use std::collections::HashSet;
use std::fs;
// Standard library imports
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use tempfile::Builder;

// This macro finds the `resources` directory at compile time and embeds
// its contents directly into the final binary.
#[derive(RustEmbed)]
#[folder = "resources/"]
struct Assets;

// A static set of known Mermaid diagram prefixes for validation.
// lazy_static ensures this is initialized only once.
lazy_static! {
    static ref MERMAID_PREFIXES: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("graph");
        s.insert("flowchart");
        s.insert("sequenceDiagram");
        s.insert("classDiagram");
        s.insert("erDiagram");
        s.insert("stateDiagram");
        s.insert("gantt");
        s.insert("pie");
        s.insert("mindmap");
        s
    };
}

/// Validates if the given text appears to be a Mermaid diagram by checking
/// the first word of the first non-empty line.
fn looks_like_mermaid(text: &str) -> bool {
    text.lines()
        .map(|line| line.trim())
        .find(|line| !line.is_empty())
        .map_or(false, |first_line| {
            first_line
                .split_whitespace()
                .next()
                .map_or(false, |prefix| MERMAID_PREFIXES.contains(prefix))
        })
}

/// Extracts all embedded assets to a temporary directory.
fn extract_assets(temp_path: &Path) -> Result<()> {
    // Extract all files from the embedded resources
    for filename in Assets::iter() {
        let asset = Assets::get(&filename).unwrap();
        let dest_path = temp_path.join(filename.as_ref());
        
        // Create parent directories if they don't exist
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).context(format!(
                "Failed to create directory {}",
                parent.display()
            ))?;
        }
        
        // Write the file
        fs::write(&dest_path, asset.data).context(format!(
            "Failed to write asset {}",
            dest_path.display()
        ))?;
        
        // Make node and mmdc executable
        if filename == "node" || filename.ends_with("mmdc") {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o755))?;
            }
        }
    }
    
    Ok(())
}

/// Renders the Mermaid source text to a PNG file using the bundled Node.js and mmdc.
fn render_mermaid(source_text: &str, output_file: &Path) -> Result<()> {
    println!("🔍 Setting up temporary directory...");
    // Create a temporary directory that will be automatically cleaned up
    let temp_dir = Builder::new()
        .prefix("mermaid-render")
        .tempdir()
        .context("Failed to create temporary directory")?;
    
    let temp_path = temp_dir.path();
    println!("📂 Temporary directory: {}", temp_path.display());
    
    // Extract all assets to the temporary directory
    println!("📦 Extracting assets...");
    extract_assets(temp_path)?;
    
    // Define paths to the executables and wrapper script
    let node_path = temp_path.join("node");
    let wrapper_script = temp_path.join("mmdc_wrapper.js");
    
    // Verify node exists and is executable
    if !node_path.exists() {
        return Err(anyhow!("Node.js binary not found at: {}", node_path.display()));
    }
    
    // Write the wrapper script to the temporary directory
    println!("📝 Writing wrapper script...");
    let wrapper_content = include_str!("mmdc_wrapper.js");
    fs::write(&wrapper_script, wrapper_content)
        .context("Failed to write wrapper script")?;
    
    // Write package.json to enable ES modules
    println!("📝 Writing package.json...");
    let package_json_path = temp_path.join("package.json");
    let package_json_content = include_str!("package.json");
    fs::write(&package_json_path, package_json_content)
        .context("Failed to write package.json")?;
    
    // Make the wrapper script executable on Unix-like systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&wrapper_script, fs::Permissions::from_mode(0o755))
            .context("Failed to set wrapper script permissions")?;
    }
    
    // Write the input to a temporary file
    println!("📝 Writing input file...");
    let input_path = temp_path.join("input.mmd");
    fs::write(&input_path, source_text)
        .context("Failed to write input file")?;
    
    // Prepare the command to run our wrapper script
    println!("🚀 Executing Mermaid CLI...");
    println!("   Node path: {}", node_path.display());
    println!("   Wrapper script: {}", wrapper_script.display());
    println!("   Input file: {}", input_path.display());
    println!("   Output file: {}", output_file.display());
    
    let output = Command::new(&node_path)
        .arg(&wrapper_script)         // Use our wrapper script
        .arg("-i").arg(&input_path)
        .arg("-o").arg(output_file)
        .current_dir(&temp_path)      // Set working directory to temp_path
        .output()
        .context("Failed to execute mmdc process")?;
    
    // Check if the command was successful
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Print the command that was run for debugging
        println!("❌ Command failed. Full command was:");
        println!("   {} {} -i {} -o {}",
            node_path.display(),
            wrapper_script.display(),
            input_path.display(),
            output_file.display()
        );
        
        return Err(anyhow!(
            "Mermaid CLI failed with status {}\n=== STDOUT ===\n{}\n=== STDERR ===\n{}",
            output.status,
            stdout,
            stderr
        ));
    }
    
    println!("✅ Mermaid CLI executed successfully");
    
    // Verify the output file was created
    if !output_file.exists() {
        return Err(anyhow!(
            "Mermaid CLI reported success but output file was not created at: {}",
            output_file.display()
        ));
    }
    
    println!("📊 Output file created: {} ({} bytes)",
        output_file.display(),
        std::fs::metadata(output_file)?.len()
    );
    
    Ok(())
}

/// Main entry point for the application.
/// Opens a file in the default application (Preview on macOS)
fn open_file_in_preview(path: &Path) -> Result<()> {
    let status = Command::new("open")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to execute 'open' command")?;
        
    if !status.success() {
        return Err(anyhow!("Failed to open file"));
    }
    
    Ok(())
}

fn main() -> Result<()> {
    // 1. Attempt to read text from the system clipboard.
    let mut clipboard = Clipboard::new()?;
    let diagram_text = match clipboard.get_text() {
        Ok(text) if looks_like_mermaid(&text) => {
            println!("📋 Found valid Mermaid diagram in clipboard.");
            text
        }
        _ => {
            // 2. If clipboard is empty or invalid, fall back to a native file picker dialog.
            println!("Clipboard does not contain a Mermaid diagram. Opening file picker...");
            let file_path = FileDialog::new()
                .add_filter("Mermaid", &["mmd", "mermaid", "txt"])
                .set_title("Select a Mermaid file")
                .pick_file();

            match file_path {
                Some(path) => fs::read_to_string(path)
                    .context("Failed to read the selected file")?,
                None => {
                    println!("🚫 No file selected. Exiting.");
                    return Ok(()); // Exit gracefully.
                }
            }
        }
    };

    // 3. Define the output file path and render the diagram.
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let output_path = current_dir.join("diagram.png");
    
    // Check if the resources are available
    if !Path::new("resources/node").exists() || !Path::new("resources/node_modules/.bin/mmdc").exists() {
        return Err(anyhow!(
            "{}",
            "Required resources not found. Please run 'setup_assets.sh' first.\n\n".to_owned()
                + "Run these commands to set up the required assets:\n"
                + "  $ chmod +x setup_assets.sh\n"
                + "  $ ./setup_assets.sh"
        ));
    }
    
    println!("🔄 Rendering diagram...");
    println!("📝 Output will be saved to: {}", output_path.display());
    
    // Ensure the output directory exists
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("Failed to create output directory")?;
        }
    }
    
    render_mermaid(&diagram_text, &output_path)
        .context("Failed to render diagram")?;

    // Verify the file was created
    if !output_path.exists() {
        return Err(anyhow!(
            "Expected output file was not created at: {}",
            output_path.display()
        ));
    }

    println!("✅ Diagram rendered successfully → {}", output_path.display());
    
    // Open the file in the default application (Preview on macOS)
    if cfg!(target_os = "macos") {
        if let Err(e) = open_file_in_preview(&output_path) {
            eprintln!("⚠️  Could not open file in Preview: {}", e);
            eprintln!("    You can find the file at: {}", output_path.display());
        } else {
            println!("👀 Opened in Preview");
        }
    }

    Ok(())
}
