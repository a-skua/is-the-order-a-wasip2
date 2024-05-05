.PHONY: all
all: init calc.wasm

.PHONY: init
init:
	npm ci

calc.wasm: src/calc.js wit/world.wit
	npx jco componentize $< --wit=$(word 2, $^) --out=$@
