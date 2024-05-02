.PHONY: run
run: component.wasm
	wasmtime run $<

component.wasm: target/wasm32-wasi/release/wasip2_stdout.wasm wasi_snapshot_preview1.command.wasm
	wasm-tools component new $< --adapt ./$(word 2, $^) -o $@

wasi_snapshot_preview1.command.wasm:
	curl -LO https://github.com/bytecodealliance/wasmtime/releases/download/v20.0.0/$@

target/wasm32-wasi/release/wasip2_stdout.wasm: src/main.rs
	cargo build --target wasm32-wasi --release
