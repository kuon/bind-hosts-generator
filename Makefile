.PHONY: build

build:
	cargo build --release

.PHONY: clean

clean:
	rm -fr target
