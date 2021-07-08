RUSTFLAGS = "-C link-arg=-s"
has_wasmopt = $(shell which wasm-opt)

all: contract.wasm.gz

contract.wasm.gz: contract.wasm
	cat $< | gzip -9 > $@

contract.wasm:
	RUSTFLAGS=$(RUSTFLAGS) cargo build --release --target wasm32-unknown-unknown
ifneq ($(has_wasmopt),)
	wasm-opt -Os ./target/wasm32-unknown-unknown/release/*.wasm -o $@
else
	cp ./target/wasm32-unknown-unknown/release/*.wasm $@
endif

.PHONY: test
test:
	cargo test

.PHONY: distclean
distclean:
	rm -rf ./contract.wasm ./contract.wasm.gz

.PHONY: clean
clean: distclean
	rm -rf ./target
