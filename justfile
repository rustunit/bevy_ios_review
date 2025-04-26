check:
    cd rust/bevy_ios_review && cargo check
    cd rust/bevy_ios_review && cargo check --target=aarch64-apple-ios

publish:
    cd rust/bevy_ios_review && cargo publish
