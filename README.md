# Mermaid Render

A desktop application built with Rust and Tauri that lets you edit and render Mermaid diagrams without needing Node.js or the Mermaid CLI installed on your system.

## Features

- Interactive Svelte interface with an editor and live preview
- Renders Mermaid diagrams from clipboard or file input
- Bundles Node.js and Mermaid CLI for a zero-dependency experience
- Cross-platform support (tested on macOS Apple Silicon)

## Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- `npm` (only needed for initial setup)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/mermaid_render.git
   cd mermaid_render
   ```

2. Run the setup script to download and prepare the required assets. The script
   uses a cache so assets are only downloaded the first time:
   ```bash
   chmod +x setup_assets.sh
   ./setup_assets.sh
   ```

3. Install the frontend dependencies and build the UI:
   ```bash
   cd frontend
   npm install
   npm run build
   cd ..
   ```
4. Build and run the application:
   ```bash
   cargo tauri dev
   ```
   For a release build, use `cargo build --release`.

The compiled binary will be available at `./target/release/mermaid_render`.

## Usage

1. Run the application:
   ```bash
   cargo tauri dev
   ```

2. Edit the diagram in the left pane and view the live preview on the right.
   Click **Export to File** to save the diagram as PNG or SVG.

## How It Works

The application bundles Node.js and the Mermaid CLI as embedded assets. When run, it:

1. Extracts the necessary files to a temporary directory
2. Uses the embedded Node.js to execute the Mermaid CLI
3. Renders the diagram to a PNG file
4. Cleans up the temporary files

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
