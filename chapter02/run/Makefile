.PHONY: run
run: component.wasm
	wasmtime run $<

.PHONY: run-wat
run-wat: component-wat.wasm
	wasmtime run $<

.PHONY: run-js
run-js: component-js.wasm
	wasmtime run $<

.PHONY: run-go
run-go: component-go.wasm
	wasmtime run $<

component.wasm: target/wasm32-wasi/release/run.wasm ../calc/target/wasm32-wasi/release/calc.wasm
	wasm-tools compose --output=$@ --definitions=$(word 2,$^) $<

component-wat.wasm: target/wasm32-wasi/release/run.wasm ../calc-wat/calc-component.wasm
	wasm-tools compose --output=$@ --definitions=$(word 2,$^) $<

component-js.wasm: target/wasm32-wasi/release/run.wasm ../calc-js/calc.wasm
	wasm-tools compose --output=$@ --definitions=$(word 2,$^) $<

component-go.wasm: target/wasm32-wasi/release/run.wasm ../calc-go/component.wasm
	wasm-tools compose --output=$@ --definitions=$(word 2,$^) $<

target/wasm32-wasi/release/run.wasm: src/main.rs wit/*.wit
	cargo component build --release

../calc/target/wasm32-wasi/release/calc.wasm:
	$(MAKE) -C ../calc

../calc-js/calc.wasm:
	$(MAKE) -C ../calc-js

../calc-go/component.wasm:
	$(MAKE) -C ../calc-go

../calc-wat/calc-component.wasm:
	$(MAKE) -C ../calc-wat

.PHONY: clean
clean:
	$(MAKE) -C ../calc clean
	cargo clean
