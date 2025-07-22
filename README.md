# Frontary-Leptos

**Reusable Leptos UI components with Tailwind CSS,** designed for real-world theming
and testing.

This repository contains:

- A Leptos component library styled using Tailwind CSS and a custom theme defined
  in `frontary-leptos`
- A demo environment to test components in **CSR (WebAssembly)** mode
- A `build.rs` is provided, which automatically generates Tailwind CSS, WASM, and
  the Axum binary **for demo purposes**.

## Repository Structure

```text
.
├── build-demo.sh                 # Script to build demo (CSR & server)
├── demo/
│   ├── csr/                      # Demo UI code for CSR WebAssembly
│   └── server/                   # Axum-based HTTP server
├── src/                          # Leptos component library
│   ├── lib.rs                    # Main library entry point (exports components)
│   └── static_files.rs           # Bundles static assets for consumers
└── static/
    ├── input.frontary.css              # Tailwind input file
    ├── tailwind.frontary.safelist.json # For classes that are dynamically referenced
    └── tailwind.frontary.theme.json    # Theme tokens for Tailwind
```

## Purpose

This project provides:

- Tailwind-themed Leptos components based on a theme defined by `frontary-leptos`
- A demo project to test and visualize your component changes interactively

## Prerequisites

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

> ⚠️ You must install these dependencies inside `demo/csr` because they are not
  used globally. Install them locally in each directory where you generate Tailwind
  CSS.
> ⚠️ We use Tailwind CSS v3 because newer versions (e.g. v4) may cause compatibility
  issues.

A `package.json` is already included, so you **do not** need to run `npm init`.
(Running `npm init -y` to generate a `package.json` is the usual first step, but
this has already been done for you.)

## Develop

To add or modify Leptos components, work in the `src` directory and make sure to
export them through `lib` so they are accessible to users. As this project uses
Tailwind CSS exclusively, please avoid introducing other design systems or custom
styles. When updating or creating components, remember to update the following files
to ensure your styles and theme tokens are included:

```text
static/input.frontary.css
static/tailwind.frontary.safelist.json
static/tailwind.frontary.theme.json
```

## Test

### Generate Tailwind CSS

```bash
npx tailwindcss -i input.css -o static/output.css --config tailwind.config.js
```

This command compiles Tailwind CSS using the `frontary-leptos` theme and outputs
it to `static/output.css`, which is referenced by `index.html`. You can run this
manually, but `build-demo.sh` will handle it automatically. It's fine to skip this
step and use `build-demo.sh` directly.

### Build the WASM and Server demo

`build-demo.sh` handles the full CSR + server build process. Key tasks include:

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

### Demo Access

Once you build with `build-demo.sh`, the Axum server will serve the demo at:

<!-- markdownlint-disable-next-line MD034 -->
> 👉 http://127.0.0.1:8446/csr/

This is powered by `demo/server`, which wraps static assets from `dist/csr`.

## About `static_files.rs`

Each user of Frontary Leptos should generate their own Tailwind CSS, including the
styles provided by Frontary Leptos. To support this, these files are distributed:

```text
static/input.frontary.css
static/tailwind.frontary.safelist.json
static/tailwind.frontary.theme.json
```

Users can use `build.rs` to import and copy these files into their local project
directory. To generate a custom Tailwind output, reference these files in your configuration
(e.g., `input.css` and `tailwind.config.js`). You can find an example setup in
the `demo` directory.

## About Tailwind Theme & Configuration

This project uses a theme-driven approach defined by `frontary-leptos`.

### Shared Library Files (used by consumers and demo)

<!-- markdownlint-disable MD013 -->
| File | Purpose |
|------|---------|
| `tailwind.frontary.theme.json` | Theme tokens: `colors`, `fontFamily`, `borderRadius`, etc. |
| `tailwind.frontary.safelist.json` | Contains style classes that are dynamically referenced and should not be purged by Tailwind. |
| `input.frontary.css` | Tailwind input with `@tailwind base/components/utilities` |
<!-- markdownlint-enable MD013 -->

These files are bundled in the Rust crate using `include_bytes!` and can be used
by external projects to build their own Tailwind setup.

### `demo/csr` Files

<!-- markdownlint-disable MD013 -->
| File | Role |
|------|------|
| `input.css` | Main input file for Tailwind build (must import `input.frontary.css`) |
| `tailwind.config.js` | Tailwind config that loads this Frontary JSON theme |
| `index.html` | References `static/output.css` like this: `<link rel="stylesheet" href="static/output.css" />` |
<!-- markdownlint-enable MD013 -->

### How to regenerate `output.css`

```bash
cd demo/csr
npx tailwindcss -i input.css -o static/output.css --config tailwind.config.js
```

This output is used directly by the WASM app.

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
- [ ] Ran `build-demo.sh` successfully
- [ ] Committed changes including `demo/csr/src`, but not `output.css` or `node_modules`
