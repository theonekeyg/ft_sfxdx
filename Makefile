RUSTFLAGS = "-C link-arg=-s"
has_wasmopt = $(shell which wasm-opt)

all: build-optimized

build-optimized: contract.wasm.gz

contract.wasm.gz: contract.wasm
	cat $< | gzip -9 > $@

contract.wasm:
	RUSTFLAGS=$(RUSTFLAGS) cargo build --release --target wasm32-unknown-unknown
ifneq ($(has_wasmopt),)
	wasm-opt -Os ./target/wasm32-unknown-unknown/release/*.wasm -o ./contract.wasm
else
	cp ./target/wasm32-unknown-unknown/release/*.wasm ./contract.wasm
endif

test:
	cargo test

distclean:
	rm -rf ./contract.wasm ./contract.wasm.gz

clean:
	rm -rf ./target ./contract.wasm ./contract.wasm.gz
