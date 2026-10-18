# starter-tauri-v3

[![CI](https://github.com/hairyf/starter-tauri-v3/actions/workflows/ci.yml/badge.svg)](https://github.com/hairyf/starter-tauri-v3/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/hairyf/starter-tauri-v3?style=flat&colorA=080f12&colorB=1fa669)](./LICENSE)

> [!WARNING]
> Experimental. Tauri v3 is still alpha (`3.0.0-alpha.5`); its API, configuration and the CEF runtime can change between releases. Pin the versions and test before using it for anything serious.

Minimal Tauri v3 starter: a vanilla TypeScript + Vite frontend, a Rust backend, one `greet` command and a `cef` branch that swaps the system webview for Chromium.

## Requirements

- Node.js 22+
- pnpm 10+
- Rust 1.95+
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

## CEF branch

```bash
git switch cef
pnpm install
pnpm tauri dev
```

The branch replaces `tauri-runtime-wry` with `tauri-runtime-cef`. The first build downloads about 1 GB of Chromium, and installers grow by roughly 170 MB. It is still the same Tauri app; only the webview runtime changes. Windows installer bundling on that branch is currently blocked upstream; the branch README has the details. GitHub's template button only copies `main`; clone the repository if you want the branch.

## CI and releases

CI runs lint, typecheck, tests, the frontend build and `cargo check` on Ubuntu. Pushing a `v*` tag builds installers for Linux, macOS and Windows and creates a draft GitHub release. Bump the version in `package.json`, `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml` first; the tag must match it.

## License

[MIT](./LICENSE) © [hairyf](https://github.com/hairyf)
