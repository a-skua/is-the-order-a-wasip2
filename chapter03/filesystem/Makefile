ARGS = Cargo.toml

.PHONY: run
run: target/wasm32-wasi/release/filesystem.wasm
	wasmtime run --dir=. $< $(ARGS)

target/wasm32-wasi/release/filesystem.wasm: src/lib.rs wit/*.wit
	cargo component build --release
