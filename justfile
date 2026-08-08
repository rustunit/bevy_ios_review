check:
    cargo c
    cargo c --target=aarch64-apple-ios
    cargo b
    cargo fmt -- --check
    cargo clippy
    cargo clippy --target=aarch64-apple-ios

publish:
    cargo publish
