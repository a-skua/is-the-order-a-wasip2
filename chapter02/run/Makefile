.PHONY: run
run: component.wasm
	wasmtime run $<

component.wasm: target/wasm32-wasi/release/run.wasm ../calc/target/wasm32-wasi/release/calc.wasm
	wasm-tools compose --output=$@ --definitions=$(word 2,$^) $<

target/wasm32-wasi/release/run.wasm: src/main.rs wit/*.wit
	cargo component build --release

../calc/target/wasm32-wasi/release/calc.wasm:
	$(MAKE) -C ../calc

.PHONY: clean
clean:
	$(MAKE) -C ../calc clean
	cargo clean
