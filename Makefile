build_merkle:
	cargo clippy
	cargo fmt 
	cargo build

test_merkle:
	cargo test
