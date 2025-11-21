# https://just.systems

[private]
default:
    @just --list

[private]
nightly-toolchain:
    rustup toolchain install nightly
    rustup component add --toolchain nightly rustfmt

[doc('Reformat Rust components')]
fmt: nightly-toolchain
    rustup run nightly cargo fmt

[doc('Check formatting of Rust components')]
check: nightly-toolchain
    rustup run nightly cargo fmt -- --check

[doc('Clean Rust components')]
clean:
    cargo clean

[doc('Test Rust components')]
test:
    cargo test --workspace

[doc('Run Clippy on all Rust packages, marking warnings as errors')]
lint:
    cargo clippy --no-deps -- -D warnings

[doc('Try to automatically fix Clippy problems')]
lint-fix:
    cargo clippy --no-deps --fix --allow-dirty
