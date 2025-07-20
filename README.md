# Frontary-Leptos

**Reusable Leptos UI components with Tailwind CSS,** designed for real-world theming
and testing.

This repository contains:

- A Leptos component library styled using Tailwind CSS and a custom theme defined
  in `frontary-leptos`
- A demo environment to test components in **CSR (WebAssembly)** mode
- Build tooling for contributors

## Repository Structure

```text
.
├── src/                # Leptos component library
├── demo/
│   ├── csr/            # Demo UI code for CSR WebAssembly
│   └── server/         # Axum-based HTTP server
```

## Purpose

This project provides:

- Tailwind-themed Leptos components based on a theme defined by `frontary-leptos`
- A demo project to test and visualize your component changes interactively

## Quickstart

### 1. Install Node (if you haven't already)

#### macOS

```bash
brew install node
```

#### Linux (Debian/Ubuntu)

```bash
sudo apt install nodejs npm
```

### 2. Install Tailwind in `demo/csr`

```bash
cd demo/csr
npm install -D tailwindcss@3 postcss autoprefixer
```

> ⚠️ We use Tailwind CSS v3 because newer versions (e.g. v4) may cause compatibility
  issues.

A `package.json` is already included. You **do not** need to run `npm init`.

### 3. Generate Tailwind CSS

```bash
npx tailwindcss -i input.css -o static/output.css --config tailwind.config.js
```

This compiles Tailwind CSS from the `frontary-leptos` theme and writes to `static/output.css`,
which is referenced in `index.html`.

### 4. Build the WASM and Server demo

```bash
./build-demo.sh
```

This will:

- Generate `output.css`
- Run `trunk build` for WASM and place final output into `dist/csr/`
- Copy `output.css` to the final output directory
- Build the Axum server

You can also run selectively:

```bash
./build-demo.sh csr
./build-demo.sh server
```

## Demo Access

Once you build with `./build-demo.sh`, the Axum server will serve the demo at:

<!-- markdownlint-disable-next-line MD034 -->
> 👉 http://127.0.0.1:8446/csr/

This is powered by `demo/server`, which wraps static assets from `dist/csr`.

## Tailwind Theme & Configuration

This project uses a theme-driven approach defined by `frontary-leptos`.

### Shared Library Files (used by consumers and demo)

<!-- markdownlint-disable MD013 -->
| File | Purpose |
|------|---------|
| `tailwind.frontary.theme.json` | Theme tokens: `colors`, `fontFamily`, `borderRadius`, etc. |
| `tailwind.frontary.safelist.json` | Required: class names to retain during purging |
| `input.frontary.css` | Tailwind input with `@tailwind base/components/utilities` |
<!-- markdownlint-enable MD013 -->

These files are bundled in the Rust crate using `include_bytes!` and can be used
by external projects to build their own Tailwind setup.

### `demo/csr` Files

<!-- markdownlint-disable MD013 -->
| File | Role |
|------|------|
| `input.css` | Main input file for Tailwind build (must import `input.frontary.css`) |
| `tailwind.config.js` | Tailwind config that loads the shared JSON theme |
| `index.html` | References `static/output.css` like this: `<link rel="stylesheet" href="static/output.css" />` |
<!-- markdownlint-enable MD013 -->

### How to regenerate `output.css`

```bash
cd demo/csr
npx tailwindcss -i input.css -o static/output.css --config tailwind.config.js
```

This output is used directly by the WASM app.

## About `build-demo.sh`

This script handles the full CSR + server build process. Key tasks include:

- Generates `output.css` from `input.css`
- Uses `trunk` to build WASM and place results into `dist/csr/`
- Copies static CSS assets into the final dist
- Builds the Axum server for demo delivery

Use it like this:

```bash
./build-demo.sh            # Build both CSR and server
./build-demo.sh csr        # Build CSR only
./build-demo.sh server     # Build server only
```

## Want to Contribute?

When you build a component in `frontary-leptos`, always add example usage under
`demo/csr/src/home.rs`. This makes it easy for you (and others) to:

- See the visual result
- Test responsiveness
- Check dark/light mode or theme variants (if any)

## Final Checklist for Contributors

- [ ] Added/modified Leptos component in `src/`
- [ ] Created usage example in `demo/csr`
- [ ] Verified styling and interaction visually
- [ ] Ran `./build-demo.sh` successfully
- [ ] Committed changes including `demo/csr/src`, but not `output.css` or `node_modules`
