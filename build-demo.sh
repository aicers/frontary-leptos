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

  # Generate output.css from input.css
  echo "🎨 Generating Tailwind CSS..."
  (
    cd demo/csr
    npx tailwindcss -i input.css -o static/output.css --config tailwind.config.js
  )

  # Build the WASM bundle with trunk
  echo "🛠 Building with Trunk..."
  mkdir -p dist/csr
  (
    cd demo/csr
    trunk build --dist ../../dist/csr --public-url="/csr/"

    # Copy output.css to dist
    echo "📦 Copying output.css to dist..."
    mkdir -p ../../dist/csr/static
    cp static/output.css ../../dist/csr/static/output.css
  )
fi

# ────────────── Build Server only ──────────────
if $BUILD_SERVER; then
  echo "🧱 Building server only (no SSR WASM)..."
  cargo build -p demo $RELEASE
fi

echo "✅ Build complete."
