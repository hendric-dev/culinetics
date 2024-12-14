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
  "editor.codeActionsOnSave": {
    "quickfix.biome": "explicit"
  },
  "editor.formatOnSave": true,
  "[json]": {
    "editor.defaultFormatter": "biomejs.biome"
  },
  "[jsonc]": {
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

As of [this issue](https://github.com/biomejs/biome/issues/2228) there is still no good monorepo support on that extension,
so it is necessary to create a symlink of the `biome.json` config into the root folder of the project. \
In the root folder run the following:

```sh
ln -s $(pwd)/frontend/biome.json $(pwd)/biome.json
```
