.PHONY: run
run: target/wasm32-wasi/release/fd_write.wasm
	wasmtime run $<

target/wasm32-wasi/release/fd_write.wasm: src/main.rs
	cargo build --target wasm32-wasi --release
