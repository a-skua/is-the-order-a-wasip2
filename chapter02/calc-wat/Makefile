calc-component.wasm: calc.wasm
	wasm-tools component new --output=$@ $<

calc.wasm: calc.wit calc.wat
	wasm-tools component embed --output=$@ $^
