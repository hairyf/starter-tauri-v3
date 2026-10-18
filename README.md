# starter-tauri-v3

[![CI](https://github.com/hairyf/starter-tauri-v3/actions/workflows/ci.yml/badge.svg)](https://github.com/hairyf/starter-tauri-v3/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/hairyf/starter-tauri-v3?style=flat&colorA=080f12&colorB=1fa669)](./LICENSE)

> [!WARNING]
> Experimental. Tauri v3 is still alpha (`3.0.0-alpha.5`) and this is the `cef` branch, which also depends on `tauri-runtime-cef 3.0.0-alpha.6`. APIs, configuration and the CEF runtime can change between releases. Pin the versions and test before using it for anything serious.

Minimal Tauri v3 starter on the Chromium Embedded Framework: a vanilla TypeScript + Vite frontend, a Rust backend and one `greet` command. The `main` branch uses the system webview through `tauri-runtime-wry`; this branch swaps in Chromium.

## Requirements

- Node.js 22+
- pnpm 10+
- Rust 1.95+
- CMake and Ninja (for the CEF build)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

## Quick start

```bash
pnpm install
pnpm tauri dev
```

Build installers:

```bash
pnpm tauri build
```

## Scripts

| Script | Description |
| --- | --- |
| `pnpm dev` | Start the Vite dev server |
| `pnpm tauri dev` | Run the desktop app |
| `pnpm build` | Build the frontend |
| `pnpm tauri build` | Build the desktop app and installers |
| `pnpm lint` | Run ESLint |
| `pnpm typecheck` | Type-check the frontend |
| `pnpm test` | Run Vitest |
| `pnpm check:rust` | Run `cargo check` |
| `pnpm check` | Run lint, typecheck, test and build |

## CEF runtime

This branch builds and runs on CEF by default:

```bash
pnpm install
pnpm tauri dev
```

The first build downloads about 1 GB of Chromium into the user cache (`~/.cache/tauri-cef` on Linux, `~/Library/Caches/tauri-cef` on macOS, `%LOCALAPPDATA%\tauri-cef` on Windows), or into `CEF_PATH` when set. Installers grow by roughly 170 MB. Chromium needs GTK 4 on Linux; the CI workflow installs the matching packages.

To build against the system webview instead, use `--no-default-features --features wry`.

### Known limitation

Windows installer bundling is blocked in the current alpha: the minimal CEF distribution that `cef-rs` downloads omits `libEGL.dll` and `libGLESv2.dll`, while the Tauri bundler expects them. `pnpm tauri dev` and `pnpm tauri build --no-bundle` work; the release workflow therefore only builds Linux and macOS for now. Copy those two DLLs from a full CEF distribution into the cache directory if you need a Windows installer today.

## CI and releases

CI runs lint, typecheck, tests, the frontend build and `cargo check` on Ubuntu. Pushing a `v*` tag builds installers for Linux and macOS and creates a draft GitHub release. Windows installers are skipped until the CEF bundling issue above is fixed upstream. Bump the version in `package.json`, `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml` first; the tag must match it.

## License

[MIT](./LICENSE) © [hairyf](https://github.com/hairyf)
