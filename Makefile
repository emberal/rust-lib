fmt:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt