# Environment Setup

## Prerequisites
- Install Rust via `rustup` and ensure the stable toolchain is available.
- Confirm build essentials and graphics dependencies are present (`pkg-config`, Vulkan/Metal drivers, X11/Wayland libraries on Linux).
- Install `cargo` binaries recommended by the project (`cargo-watch`, `cross`) as needed.

## Project Initialization
- Clone the repository and run `rustup show` to verify the stable toolchain.
- Execute `cargo fetch` to pre-download dependencies and validate network access.
- Run `cargo fmt` and `cargo check` to confirm the toolchain is working locally.

## Development Workflow
- Use `cargo fmt` before committing to enforce formatting.
- Run `cargo lint` (alias for `cargo clippy -- -D warnings`) during development to catch issues early.
- `cargo run` launches the application; assets hot-reload when run in debug mode.
- For iterative development, `cargo watch -x run` provides live rebuilds.
