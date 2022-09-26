push: test
	git push

check:
	cargo check

test: fmt_check check
	cargo test

fmt:
	cargo fmt

fmt_check: fmt
	cargo clippy -- -D warnings
	cargo fmt -- --check

audit:
	cargo audit