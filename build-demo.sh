#!/bin/bash
set -e

cd "$(dirname "$0")" # ensure we're in project root

BUILD_CSR=false
BUILD_SERVER=false

TARGET_DIR="dist"
RELEASE=  # optionally set this to "--release"

# ────────────── Argument dispatch ──────────────
case "$1" in
  "csr")
    BUILD_CSR=true
    ;;
  "server")
    BUILD_SERVER=true
    ;;
  "")
    BUILD_CSR=true
    BUILD_SERVER=true
    ;;
  *)
    echo "❌ Unknown argument: '$1'"
    echo "Usage: $0 [csr|server]"
    exit 1
    ;;
esac


# ────────────── Build CSR ──────────────
if $BUILD_CSR; then
  echo "🚀 Building CSR with trunk..."

  # Clean dist/csr before rebuilding
  echo "🧹 Cleaning previous dist/csr output..."
  rm -rf dist/csr

  # Install Tailwind CSS if not already installed
  echo "📦 Checking Tailwind CSS installation in demo/csr/..."
  cd demo/csr

  if [ ! -d "node_modules" ] || [ ! -d "node_modules/tailwindcss" ]; then
      echo "📦 Installing Tailwind CSS dependencies..."
      npm install -D tailwindcss@3 postcss autoprefixer || {
          echo "❌ Failed to install Tailwind CSS"
          exit 1
      }
  else
      echo "✅ Tailwind CSS already installed. Skipping npm install."
  fi

  # Generate output.css from input.css
  echo "🎨 Generating Tailwind CSS..."
  npx tailwindcss -i input.css -o static/output.css --config tailwind.config.js || {
      echo "❌ Failed to generate output.css"
      exit 1
  }

  # Build the WASM bundle with trunk
  echo "🛠 Building with Trunk..."
  mkdir -p ../../dist/csr
  trunk build --dist ../../dist/csr --public-url="/csr/" || {
      echo "❌ Trunk build failed"
      exit 1
  }

  # Copy output.css to dist
  echo "📦 Copying output.css to dist..."
  mkdir -p ../../dist/csr/static
  cp static/output.css ../../dist/csr/static/output.css || {
      echo "❌ Failed to copy output.css"
      exit 1
  }

  cd ../.. # return to project root
fi

# ────────────── Build Server only ──────────────
if $BUILD_SERVER; then
  echo "🧱 Building server only (no SSR WASM)..."
  cargo build -p demo $RELEASE
fi

echo "✅ Build complete."
