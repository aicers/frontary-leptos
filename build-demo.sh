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
  *)
    BUILD_CSR=true
    ;;
esac

# ────────────── Build CSR ──────────────
if $BUILD_CSR; then
  echo "🚀 Building CSR with trunk..."
  mkdir -p dist/csr
  (
    cd demo/csr
    trunk build --dist ../../dist/csr --public-url="/csr/"
  )
fi

# ────────────── Build Server only ──────────────
if $BUILD_SERVER; then
  echo "🧱 Building server only (no SSR WASM)..."
  cargo build -p demo $RELEASE
fi

echo "✅ Build complete."
