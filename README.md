# Mermaid Render

A standalone Rust application that renders Mermaid diagrams to PNG files without requiring Node.js or Mermaid CLI to be installed on the system.

## Features

- Renders Mermaid diagrams from clipboard or file input
- Bundles Node.js and Mermaid CLI for a zero-dependency experience
- Simple command-line interface
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
4. Build the release binary:
   ```bash
   cargo build --release
   ```

The binary will be available at `./target/release/mermaid_render`.

## Usage

### From Clipboard
1. Copy a Mermaid diagram to your clipboard, for example:
   ```mermaid
   graph TD;
       A-->B;
       B-->C;
       C-->A;
   ```

2. Run the application:
   ```bash
   ./target/release/mermaid_render
   ```

3. The rendered diagram will be saved as `diagram.png` in the current directory.

### From File
```bash
echo 'graph TD; A-->B; B-->C; C-->A;' > example.mmd
./target/release/mermaid_render --input example.mmd --output output.png
```

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
