#!/bin/sh
set -e # Exit immediately if a command exits with a non-zero status.

# --- Configuration ---
NODE_VERSION="22.5.1" # Use a known recent LTS or Mainline version
RESOURCES_DIR="resources"
NPM_TMP_DIR="npm_tmp"

# --- DO NOT EDIT BELOW ---
NODE_ARCH="arm64"
NODE_FILENAME="node-v${NODE_VERSION}-darwin-${NODE_ARCH}"
NODE_URL="https://nodejs.org/dist/v${NODE_VERSION}/${NODE_FILENAME}.tar.gz"

echo "--- Preparing assets for mermaid_render ---"

# 1. Create the resources directory
echo "1. Creating './${RESOURCES_DIR}' directory..."
if [ -f "${RESOURCES_DIR}/node" ] && [ -f "${RESOURCES_DIR}/@mermaid-js/mermaid-cli/dist/cli.js" ]; then
    echo "Resources already present. Skipping download."
    exit 0
fi
rm -rf "${RESOURCES_DIR}"  # Clean up any existing resources
mkdir -p "${RESOURCES_DIR}"

# 2. Download and extract Node.js for Apple Silicon
echo "2. Downloading Node.js v${NODE_VERSION} for ${NODE_ARCH}..."
curl -L "${NODE_URL}" -o "${NODE_FILENAME}.tar.gz"

echo "   Extracting Node.js..."
mkdir -p "${RESOURCES_DIR}/nodejs"
tar -xzf "${NODE_FILENAME}.tar.gz" -C "${RESOURCES_DIR}/nodejs" --strip-components=1

# Create symlinks to node and npm in the resources directory
ln -sf "${PWD}/${RESOURCES_DIR}/nodejs/bin/node" "${RESOURCES_DIR}/node"
ln -sf "${PWD}/${RESOURCES_DIR}/nodejs/bin/npm" "${RESOURCES_DIR}/npm"

# Add node and npm to PATH
export PATH="${PWD}/${RESOURCES_DIR}:${PATH}"

# 3. Install Mermaid CLI globally in a temporary directory
echo "3. Installing Mermaid CLI..."
mkdir -p "${NPM_TMP_DIR}"

# Install Mermaid CLI with all its dependencies
echo "   Running npm install..."
"${RESOURCES_DIR}/npm" install -g --prefix "${NPM_TMP_DIR}" @mermaid-js/mermaid-cli --no-save --no-package-lock --no-fund --no-audit

# Copy the installed files to our resources directory
echo "   Copying Mermaid CLI files to resources..."
mkdir -p "${RESOURCES_DIR}/@mermaid-js"
cp -r "${NPM_TMP_DIR}/lib/node_modules/@mermaid-js/mermaid-cli" "${RESOURCES_DIR}/@mermaid-js/"

# 4. Verify the installation
echo "4. Verifying installation..."
if [ ! -f "${RESOURCES_DIR}/@mermaid-js/mermaid-cli/dist/cli.js" ]; then
    echo "Error: Mermaid CLI installation is incomplete!"
    echo "Expected file not found: ${RESOURCES_DIR}/@mermaid-js/mermaid-cli/dist/cli.js"
    echo "Files in mermaid-cli directory:"
    find "${RESOURCES_DIR}/@mermaid-js/mermaid-cli" -type f | sort
    exit 1
fi

# 5. Clean up intermediate files
echo "5. Cleaning up temporary files..."
rm -f "${NODE_FILENAME}.tar.gz"
rm -rf "${NPM_TMP_DIR}"

# 6. Make node and npm executable
chmod +x "${RESOURCES_DIR}/node"
chmod +x "${RESOURCES_DIR}/npm"

# 7. Create a simple test script for verification
cat > "${RESOURCES_DIR}/test-mermaid.js" << 'EOL'
const { execFile } = require('child_process');
const path = require('path');

const mmdcPath = path.join(__dirname, 'node_modules', '@mermaid-js', 'mermaid-cli', 'dist', 'cli.js');
const nodePath = path.join(__dirname, 'node');

console.log('Testing Mermaid CLI installation...');
console.log(`Node path: ${nodePath}`);
console.log(`MMDC path: ${mmdcPath}`);

if (!require('fs').existsSync(mmdcPath)) {
    console.error('Error: Mermaid CLI not found at expected location');
    process.exit(1);
}

console.log('Mermaid CLI found!');
EOL

echo "✅ Assets prepared successfully in './${RESOURCES_DIR}'."
echo "   Node.js version: $("${RESOURCES_DIR}/node" --version)"
echo "   Mermaid CLI files:"
find "${RESOURCES_DIR}/@mermaid-js/mermaid-cli/dist" -type f | sort
