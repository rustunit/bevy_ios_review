# bevy_ios_review

[![crates.io](https://img.shields.io/crates/v/bevy_ios_review.svg)](https://crates.io/crates/bevy_ios_review)

Rust crate and Swift package to easily integrate iOS's `requestReview` API into a Bevy application.

![example](./assets/example.png)
example screenshot from our game using this crate: [zoolitaire.com](https://zoolitaire.com)

**See also** [bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap), [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications), [bevy_ios_gamecenter](https://github.com/rustunit/bevy_ios_gamecenter), [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts) & [bevy_ios_impact](https://github.com/rustunit/bevy_ios_impact)

## Instructions

1. Add to XCode: Add SPM (Swift Package Manager) dependency
2. Add Rust dependency
3. Setup Plugin

### 1. Add to XCode

Go to `File` -> `Add Package Dependencies` and paste `https://github.com/rustunit/bevy_ios_review.git` into the search bar on the top right:
![xcode](./assets/xcode-spm.png)

### 2. Add Rust dependency

```
cargo add bevy_ios_review
``` 

or 

```
bevy_ios_review = { version = "0.1" }
```

### 3. Setup Plugin

Initialize Bevy Plugin:

```rust
app.add_plugins(bevy_ios_review::IosRequestReviewPlugin);
```

Trigger Review Event in your application code:

```rust
fn some_system(mut event: EventWriter<IosRequestReview>) {
    event.send(IosRequestReview);
}
```

## Bevy version support

|bevy|bevy\_ios\_review|
|----|---|
|0.14|0.2,main|
|0.13|0.1|

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
