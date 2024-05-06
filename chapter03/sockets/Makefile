.PHONY: run
run: target/wasm32-wasi/release/sockets.wasm
	wasmtime run -S=inherit-network $<

target/wasm32-wasi/release/sockets.wasm: src/lib.rs wit/*.wit
	cargo component build --release
