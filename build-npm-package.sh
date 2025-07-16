#!/bin/bash

set -e  # Exit on any error

echo "🧹 Cleaning previous builds..."
rm -rf npx-cli/dist
mkdir -p npx-cli/dist/macos-arm64

echo "🔨 Building frontend..."
npm run frontend:build

echo "🔨 Building Rust binaries..."
cargo build --release --manifest-path backend/Cargo.toml
cargo build --release --bin mcp_task_server --manifest-path backend/Cargo.toml

echo "📦 Creating distribution package..."

# Copy the main binary
cp target/release/toolflow toolflow
cp target/release/mcp_task_server toolflow-mcp

zip toolflow.zip toolflow
zip toolflow-mcp.zip toolflow-mcp

rm toolflow toolflow-mcp

mv toolflow.zip npx-cli/dist/macos-arm64/toolflow.zip
mv toolflow-mcp.zip npx-cli/dist/macos-arm64/toolflow-mcp.zip

echo "✅ NPM package ready!"
echo "📁 Files created:"
echo "   - npx-cli/dist/macos-arm64/toolflow.zip"
echo "   - npx-cli/dist/macos-arm64/toolflow-mcp.zip"