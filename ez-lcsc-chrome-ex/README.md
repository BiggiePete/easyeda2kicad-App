# sv

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
# ez-lcsc-chrome-ex

This folder contains the Svelte-based Chrome extension that integrates with LCSC pages.

Quick commands

Install dependencies:

```bash
cd ez-lcsc-chrome-ex
npm ci
```

Run in development (hot reload):

```bash
npm run dev
```

Build production bundle:

```bash
npm run build
```

The production build output is written to `build/`. The release workflows will zip this folder and attach it to the GitHub Release.

Notes

- If you use `pnpm` or `yarn`, replace `npm ci` with the appropriate install command.
- The extension manifest and Svelte routing are in this folder; edit source files under `src/`.
