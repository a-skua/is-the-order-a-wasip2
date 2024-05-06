.PHONY: run
run: target/wasm32-wasi/release/random.wasm
	wasmtime run $<

target/wasm32-wasi/release/random.wasm: src/lib.rs wit/*.wit
	cargo component build --release
