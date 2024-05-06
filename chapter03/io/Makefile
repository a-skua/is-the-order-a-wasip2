.PHONY: run
run: target/wasm32-wasi/release/io.wasm
	wasmtime run $<

target/wasm32-wasi/release/io.wasm: src/lib.rs wit/*.wit
	cargo component build --release
