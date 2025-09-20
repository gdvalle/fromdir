cargo_target := "x86_64-unknown-linux-gnu"

build:
	RUSTFLAGS="-Zfmt-debug=none -Zlocation-detail=none" \
	    cargo +nightly build \
	        -Z build-std=std,panic_abort \
	        -Z build-std-features=optimize_for_size \
	        -Z build-std-features=panic_immediate_abort \
	        --release \
	        --target={{cargo_target}}

test:
	cargo test

check:
	cargo clippy -- -D clippy::pedantic
	cargo fmt -- --check

fmt:
	cargo fmt
