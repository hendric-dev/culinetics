# Culinetics Frontend

## Prerequisites

- [Bun](https://bun.sh/)

## Usage

1. Install the dependencies:

```sh
bun install
```

2. Start the application:

```sh
bun dev
```

Shortly after, the UI can be accessed on http://localhost:8080

## Formatting & Linting

We are using [Biome](https://biomejs.dev/) as formatter & linter.

### VSCode Setup

To integrate Biome into VSCode install the [Biome Extension](https://marketplace.visualstudio.com/items?itemName=biomejs.biome)
and use the following workspace settings:

```json
{
  "biome.lspBin": "./frontend/node_modules/@biomejs/biome/bin/biome",
  "editor.formatOnSave": true,
  "[json]": {
    "editor.defaultFormatter": "biomejs.biome"
  },
  "[typescript]": {
    "editor.defaultFormatter": "biomejs.biome"
  },
  "[vue]": {
    "editor.defaultFormatter": "biomejs.biome"
  }
}
```
