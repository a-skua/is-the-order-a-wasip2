.PHONY: run
run: target/wasm32-wasi/release/clocks.wasm
	wasmtime run $<

target/wasm32-wasi/release/clocks.wasm: src/lib.rs wit/*.wit
	cargo component build --release
