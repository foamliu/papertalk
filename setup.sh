#!/bin/bash

echo "========================================"
echo "PaperTalk Setup Script"
echo "========================================"
echo

echo "Checking for Node.js..."
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed."
    echo "Please install Node.js from: https://nodejs.org/"
    echo "Then run this script again."
    exit 1
fi

echo "Node.js found: $(node --version)"
echo

echo "Checking for Rust..."
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed."
    echo "Please install Rust from: https://rustup.rs/"
    echo "Then run this script again."
    exit 1
fi

echo "Rust found: $(rustc --version)"
echo

echo "Installing npm dependencies..."
npm install
if [ $? -ne 0 ]; then
    echo "Failed to install npm dependencies."
    exit 1
fi

echo
echo "========================================"
echo "Setup completed successfully!"
echo "========================================"
echo
echo "Next steps:"
echo "1. Install Ollama from https://ollama.ai/"
echo "2. Download the model: ollama pull qwen3:8b-q4_K_M"
echo "3. Run the app: npm run tauri:dev"
echo
