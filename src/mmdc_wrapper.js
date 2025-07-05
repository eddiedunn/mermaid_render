#!/usr/bin/env node
'use strict';

const path = require('path');

// Calculate the path to the Mermaid CLI
const mermaidCliPath = path.join(__dirname, 'node_modules', '@mermaid-js', 'mermaid-cli', 'src', 'index.js');

// Use dynamic import to load the Mermaid CLI as an ES module
import('file://' + mermaidCliPath)
  .then(module => {
    const { cli } = module;
    // Call the CLI with the provided arguments
    cli(process.argv.slice(2));
  })
  .catch(err => {
    console.error('Failed to load Mermaid CLI:', err);
    console.error('Attempted path:', mermaidCliPath);
    process.exit(1);
  });
