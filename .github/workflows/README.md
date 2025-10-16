# This folder contains GitHub Actions workflows for release builds

Workflows:

- build_release_windows.yml: Triggered on tag push matching `V*`. Builds the Chrome extension (in `ez-lcsc-chrome-ex`), zips it, builds the Tauri desktop connector in `ez-lcsc-desktop-connector` for Windows, copies the generated MSI to the repository root, and uploads both artifacts to the GitHub Release.
- build_release_linux.yml: Triggered on tag push matching `V*`. Builds the Chrome extension, zips it, builds the Tauri desktop connector for Linux and uploads the installer and extension to the GitHub Release.

Notes and prerequisites:

- The workflows use `npm ci` and `npm run tauri build`. Ensure `package.json` scripts exist and produce the expected outputs.
- Tauri builds require additional system dependencies on runner images. The workflow uses the standard `ubuntu-latest` and `windows-latest` images which include Rust. If the Tauri build requires specific native packages (e.g., libgtk, build-essential), modify the workflow to install them before `npm run tauri build`.
- The workflows use the automatically provided `GITHUB_TOKEN`. No additional secrets are required to upload release assets. If you use third-party actions that need tokens, set them in repository secrets.
- If you prefer to use a different release action or to create a draft release first, edit the workflows accordingly.
