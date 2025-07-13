#!/bin/bash
set -e

# Configuration
RESOURCES_DIR="resources"
NODE_VERSION="22.5.1"
NODE_ARCH="arm64"
NODE_FILENAME="node-v${NODE_VERSION}-darwin-${NODE_ARCH}"
NODE_URL="https://nodejs.org/dist/v${NODE_VERSION}/${NODE_FILENAME}.tar.gz"

# Clean up and create directories
echo "1. Setting up directories..."
if [ -f "${RESOURCES_DIR}/node" ] && [ -f "${RESOURCES_DIR}/node_modules/.bin/mmdc" ]; then
    echo "Resources already present. Skipping download."
    exit 0
fi
rm -rf "${RESOURCES_DIR}"
mkdir -p "${RESOURCES_DIR}/nodejs"

# Download and extract Node.js
echo "2. Downloading Node.js v${NODE_VERSION}..."
curl -L "${NODE_URL}" -o "${NODE_FILENAME}.tar.gz"
tar -xzf "${NODE_FILENAME}.tar.gz" -C "${RESOURCES_DIR}/nodejs" --strip-components=1

# Create symlinks
echo "3. Setting up symlinks..."
ln -sf "${PWD}/${RESOURCES_DIR}/nodejs/bin/node" "${RESOURCES_DIR}/node"
ln -sf "${PWD}/${RESOURCES_DIR}/nodejs/bin/npm" "${RESOURCES_DIR}/npm"

# Make binaries executable
chmod +x "${RESOURCES_DIR}/node"
chmod +x "${RESOURCES_DIR}/npm"

# Set up environment to use our Node.js
export PATH="${PWD}/${RESOURCES_DIR}:${PATH}"
export NPM_CONFIG_PREFIX="${PWD}/${RESOURCES_DIR}/npm_global"

# Verify Node.js version
echo "4. Verifying Node.js installation..."
"${RESOURCES_DIR}/node" --version

# Install Mermaid CLI locally
echo "5. Installing Mermaid CLI..."
mkdir -p "${RESOURCES_DIR}/node_modules"
cd "${RESOURCES_DIR}"
./npm install @mermaid-js/mermaid-cli --no-save --no-package-lock --no-fund --no-audit
cd ..

# Verify Mermaid CLI installation
echo "6. Verifying Mermaid CLI installation..."
if [ -f "${RESOURCES_DIR}/node_modules/.bin/mmdc" ]; then
    echo "✅ Mermaid CLI installed successfully!"
else
    echo "❌ Failed to install Mermaid CLI"
    exit 1
fi

# Clean up
echo "7. Cleaning up..."
rm -f "${NODE_FILENAME}.tar.gz"

# Create a test script
cat > "${RESOURCES_DIR}/test-mermaid.js" << 'EOL'
const { execFile } = require('child_process');
const path = require('path');
const fs = require('fs');

const nodePath = path.join(__dirname, 'node');
const mmdcPath = path.join(__dirname, 'node_modules', '.bin', 'mmdc');

console.log('Testing Mermaid CLI installation...');
console.log(`Node path: ${nodePath}`);
console.log(`MMDC path: ${mmdcPath}`);

if (!fs.existsSync(mmdcPath)) {
    console.error('Error: Mermaid CLI not found at expected location');
    process.exit(1);
}

console.log('✅ Mermaid CLI found!');
EOL

echo "✅ Setup completed successfully!"
echo "   Node.js version: $(${RESOURCES_DIR}/node --version)"
echo "   Mermaid CLI: ${RESOURCES_DIR}/node_modules/.bin/mmdc"
