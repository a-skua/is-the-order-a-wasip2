component.wasm: calc.embed.wasm wasi_snapshot_preview1.reactor.wasm
	wasm-tools component new $< --adapt ./$(word 2, $^) -o $@

calc.embed.wasm: calc.wit calc.wasm
	wasm-tools component embed $^ -o $@

calc.wasm: calc.go
	tinygo build -o=$@ -target=wasi $<

wasi_snapshot_preview1.reactor.wasm:
	curl -LO https://github.com/bytecodealliance/wasmtime/releases/download/v20.0.0/$@
