all: check build test

clean:
	rm -rf Cargo.lock
	rm -rf target
	rm -rf tests/test_k8s
	git checkout tests/test_k8s
	rm -rf tests/test_pet

prepare:
	rustup override set stable
	rustup component add rustfmt
	rustup component add clippy
	rustup toolchain install nightly --allow-downgrade -c rustfmt clippy

check:
	cargo +nightly fmt --all
	cargo clippy --all --features "actix" -- -D clippy::all

check_nightly:
	cargo +nightly fmt --all
	cargo +nightly clippy --all --features "actix" -- -D clippy::all

doc:
	cargo doc --all --all-features --no-deps

build:
	cargo build
	cargo build --features actix
	cargo build --features cli

test:
	cargo test --all --features "actix cli chrono uuid actix-validator swagger-ui"

	# We test this one seperately as it affects the generated spec, which'd fail the other tests
	cargo test test_module_path_in_definition_name --features "actix cli chrono uuid swagger-ui path-in-definition"
	# Compile the code generated through tests.
	cd tests/test_pet && cargo check
	cd tests/test_pet/cli && CARGO_TARGET_DIR=../target cargo check
	cd tests/test_k8s && cargo check
	cd tests/test_k8s/cli && CARGO_TARGET_DIR=../target cargo check
	# Test that the CLI runs successfully.
	# ./tests/test_k8s/target/debug/test-k8s-cli --help > /dev/null
