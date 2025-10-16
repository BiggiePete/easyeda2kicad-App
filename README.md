# EZ_LCSC2KiCAD

A small toolkit to help create KiCad projects by pulling parts and metadata from LCSC.

This repository contains two main pieces:

- `ez-lcsc-chrome-ex` — a browser extension (Svelte) used to scrape or integrate with LCSC pages.
- `ez-lcsc-desktop-connector` — a Tauri desktop app that helps import parts into KiCad projects and manage local data.

Quick links

- Chrome extension: `ez-lcsc-chrome-ex`
- Desktop connector: `ez-lcsc-desktop-connector`
- CI/release workflows: `.github/workflows`

Quick start

1. Clone the repository:

```powershell
git clone <repo-url>
cd EZ_LCSC2KiCAD
```

2. Build the production Chrome extension:

```powershell
npm i
npm run build
```

3. Build the desktop connector (Windows example):

```powershell
cd ..\ez-lcsc-desktop-connector
npm i
npm run tauri build
```

Releases

- Pushing a git tag matching `V*` (for example `V1.0.0`) triggers CI workflows that will build the extension and the desktop connector and attach artifacts (chrome .zip, Windows .msi, Linux installer) to a GitHub Release.

Notes

- Tauri builds may require additional native libraries on the build runners (especially on Linux). If you see build failures in CI, add the required native packages to the workflow before running `npm run tauri build`.

If you'd like a shorter walkthrough or screenshots for installation, tell me what you'd prefer and I can add them.
