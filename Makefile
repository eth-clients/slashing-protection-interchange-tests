check:
	cargo run --manifest-path schema_validator/Cargo.toml --release -- schema.json tests/generated
