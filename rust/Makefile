crate_name=rust
crate_root=dist

setup:
# Install wasm-pack https://rustwasm.github.io/wasm-pack/installer/
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
# Add wasm tier 2 wasm target https://doc.rust-lang.org/nightly/rustc/platform-support.html
# rustup target add wasm32-unknown-unknown

dev:
# cargo build --lib --release --target wasm32-unknown-unknown
# wasm-bindgen "./target/wasm32-unknown-unknown/release/${crate_name}.wasm" --out-dir "${crate_root}/pkg" --typescript --target bundler
# everything done in wasm pack output in pkg
# wasm-pack build --target bundler --out-dir dist
	wasm-pack build --target web --out-dir="./dist" --out-name="main" --dev

dev-replace: dev
	yes | cp -rf ./dist/* ../svelte/node_modules/local-rust


build-release:
	wasm-pack build --target web --out-dir="./dist" --out-name="main" --release
