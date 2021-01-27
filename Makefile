VERSION:=$(shell git describe --tags)

check:
	cargo run --manifest-path schema_validator/Cargo.toml --release -- schema.json tests/generated

release:
	tar -czvf eip-3076-tests-$(VERSION).tar.gz tests/
