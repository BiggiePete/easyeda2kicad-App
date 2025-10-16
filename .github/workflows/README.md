# GitHub Actions workflows (release builds)

This folder contains CI workflows used to build the Chrome extension and the Tauri desktop connector and attach the resulting artifacts to a GitHub Release.

Workflows

- `build_release_windows.yml` — Triggered on tag push matching `V*`. Builds the Chrome extension (in `ez-lcsc-chrome-ex`), zips it, builds the Tauri desktop connector for Windows, copies the MSI to the repo root, and uploads both artifacts to the release.
- `build_release_linux.yml` — Triggered on tag push matching `V*`. Builds the Chrome extension and the Tauri desktop connector for Linux and uploads the extension zip and the Linux installer to the release.

Notes and prerequisites

- The workflows use `npm ci` and `npm run tauri build`. Make sure `package.json` scripts exist and produce the expected outputs.
- Tauri builds may require additional native packages on the runner (for example `libgtk-3-dev` on Linux). Add installation steps to the workflow if needed.
- The built-in `GITHUB_TOKEN` is used to create releases and upload assets; no additional repository secrets are required for that.

If you want the release to be created as a draft, or to customize asset names, edit the workflows accordingly.
