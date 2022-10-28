# DT3 WASM Autosplitter

This is a new wasm version autosplitter for livesplit for Distorted Travesty 3.

It is waiting on Settings support to be fully added to livesplit. Until then use the ASL autosplitter currently in livesplit.

## Building

```
rustup target add wasm32-unknown-unknown
cargo build --release
```

## Usage

1. Use livesplit version 1.8.22 or higher
2. Edit layout -> Control -> Auto splitting Runtime
3. Set the path to be something like `repo/target/wasm32-unknown-unknown/dt3_autosplitter.wasm`
4. You'll need to fully restart livesplit every time it is built for the changes to be seen.
5. View log output using DebugView.