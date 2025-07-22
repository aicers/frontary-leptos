const frontaryTheme = require("../../static/tailwind.frontary.theme.json");

module.exports = {
    content: [
        "./index.html", // static HTML
        "./src/**/*.{rs,html}", // Leptos + WASM Rust files
    ],
    theme: {
        extend: frontaryTheme,
    },
    plugins: [],
};
