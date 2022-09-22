

push: test audit fmt_check
	git push

check:
	cargo check

test: fmt_check check
	cargo test

fmt:
	cargo fmt

fmt_check:
	cargo clippy -- -D warnings
	cargo fmt -- --check

audit:
	cargo audit