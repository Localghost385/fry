build-binary:
	cargo build --release

gen-wasm-for-extension:
	rm -rf ./page/src/lib/wasm
	wasm-pack build --target web --out-dir ./page/src/lib/wasm --release