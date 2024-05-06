.PHONY: run
run: target/wasm32-wasi/release/cli.wasm
	wasmtime run $<

target/wasm32-wasi/release/cli.wasm: src/lib.rs wit/*.wit
	cargo component build --release
