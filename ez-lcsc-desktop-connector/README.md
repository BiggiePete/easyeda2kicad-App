# ez-lcsc-desktop-connector

The desktop connector is a Svelte + Tauri application that:

- Lets you import part and project files into the local workspace.
- Runs a small background server used by the Chrome extension to communicate with the desktop app.

Quick start

Install dependencies and run in development mode:

```bash
cd ez-lcsc-desktop-connector
npm i
npm run tauri dev
```

Build release artifacts (MSI for Windows, .deb/.AppImage for Linux):

```bash
npm i
npm run tauri build
```

Output

- Release artifacts are placed under `src-tauri/target/release/bundle/`.
- The CI workflows locate these artifacts and attach them to the GitHub Release when you push a tag matching `V*`.

Notes

- The desktop app also starts a local background server process that the Chrome extension can call to hand-off data or trigger imports. Keep the desktop app running if you want the extension to be able to import directly.
