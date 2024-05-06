.PHONY: serve
serve: target/wasm32-wasi/release/http.wasm
	wasmtime serve -S=inherit-network $<

target/wasm32-wasi/release/http.wasm: src/lib.rs wit/*.wit
	cargo component build --release
