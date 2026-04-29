# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rust workspace organized as `base/ + crates/ + examples/ + xtask/`.

- `base/`: foundational crates such as `error`, `log`, `types`, and `utils`
- `crates/`: main layered crates: `model`, `core`, `platform-std`, `runtime`, `sdk`
- `examples/rust-demo/`: desktop demo app, Slint UI assets, i18n files, packaging config
- `xtask/`: workspace automation entrypoint for checks, builds, and packaging

Keep responsibilities narrow: reusable value objects belong in `base/types`, public domain models in `crates/model`, orchestration in `crates/core`, and concrete runtime/platform code outside core.

## Build, Test, and Development Commands

- `just fmt`: format the whole workspace with `rustfmt`
- `just check`: run workspace checks through `cargo xtask check`
- `just test`: run workspace tests through `cargo xtask test`
- `just lint`: run Clippy with workspace settings
- `just verify`: run the full local gate before committing
- `just run-demo`: launch the desktop demo

Direct Cargo equivalents are also valid, for example:

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Coding Style & Naming Conventions

Use Rust 2024 edition defaults and `cargo fmt --all`. Prefer small, explicit modules over generic frameworks. Public types use `UpperCamelCase`; modules and files use `snake_case`. Keep APIs stable and layer-appropriate: avoid placing `tokio`, sockets, `SystemTime`, or platform DTOs in `model` or `core` unless the layer explicitly owns them.

## Testing Guidelines

Place unit tests close to the code under `#[cfg(test)]`. Test behavior, not implementation details. For core logic, prefer fakes over real network or filesystem access. Run `just test` or `cargo test --workspace` before opening a PR.

## Commit & Pull Request Guidelines

Recent history uses short, scope-first commit messages such as `core`, `model`, `进一步修改 model`, and `初步实现 types`. Follow that style: keep subjects short, specific, and focused on one area. For PRs, include:

- what changed
- why it changed
- affected crates or example apps
- screenshots for Slint UI changes
- verification commands run locally

## Security & Configuration Tips

Do not commit secrets, signing material, or machine-specific paths. Keep packaging and demo configuration under `examples/rust-demo/` reproducible and prefer workspace-level commands over ad hoc scripts.
