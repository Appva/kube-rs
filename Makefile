VERSION=$(shell grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)

clippy:
	#rustup component add clippy --toolchain nightly
	touch src/lib.rs
	cargo +nightly clippy --no-default-features --features=openapi,rustls-tls

fmt:
	#rustup component add rustfmt --toolchain nightly
	cargo +nightly fmt

doc:
	cargo doc --lib
	xdg-open target/doc/kube/index.html

test:
	cargo test --lib
	cargo test --example crd_api crd_reflector
	cargo test --features=openapi -j4
	cargo test --lib --no-default-features --features=rustls-tls

readme:
	rustdoc README.md --test --edition=2018

.PHONY: doc build fmt clippy test readme
