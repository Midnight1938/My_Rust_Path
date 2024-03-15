# Asteroids game

This is a basic asteroids implementation of the Asteroids game running on the bevy game engine

## Building

### Local
```bash
cargo run
```
### Web Build

This is only for building deployments
```bash
rustup target add wasm32-unknown-unknown 
cargo install -f wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown                     
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "bevy-asteroid" \
    ./target/wasm32-unknown-unknown/release/bevy-asteroid.wasm
```
Next steps:
1. create an [index.html](index.html) (or any other wasm renderer) in the `out/` folder.
2. Copy over the `assets/` folder.
## Known issues
- [ ] No Restart options
- [ ] Slow runs on firefox
- [ ] Basic