<div align="center">
  <h1>
    <a href="https://hendric-dev.github.io/culinetics">🍲  Culinetics</a>
  </h1>
</div>

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en) (to install [Tailwind CSS](https://tailwindcss.com/docs/installation))

## Usage

Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

The application is started on http://localhost:8080

## VSCode Setup

Recommended extensions:

- [Dioxus](https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus)
- [Tailwind CSS](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Configuration:

```json
{
  "tailwindCSS.experimental.classRegex": [
    "class\\s*:\\s*\"([^\"]*)"
  ],
  "tailwindCSS.includeLanguages": {
    "rust": "html"
  }
}
```
