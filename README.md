# dingtalk-sdk

A Rust SDK for DingTalk Open Platform APIs.

> Warning: This is not an official DingTalk or Alibaba crate. It is a community-maintained project.

## Status

This crate currently provides:

- A shared `DingTalkClient` with token handling and typed request helpers.
- A broad set of endpoint modules matching DingTalk API domains.
- Shared request/response models in `dingtalk_sdk::models`.
- Integration tests using `wiremock`.

## Installation

`crates.io` publish is not completed yet. Add it from Git for now:

```toml
[dependencies]
dingtalk-sdk = { git = "https://github.com/palpo-im/dingtalk-sdk" }
```

## Quick Start

```rust
use dingtalk_sdk::DingTalkClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DingTalkClient::new()?;
    client.set_credentials("app_key".to_string(), "app_secret".to_string());

    let token = client.get_access_token().await?;
    println!("token: {token}");
    Ok(())
}
```

## Examples

- `examples/basic_client.rs`
- `examples/send_im_message.rs`

Run:

```bash
cargo run --example basic_client
cargo run --example send_im_message
```

## Development

```bash
cargo fmt
cargo check
cargo test
```

## License

Licensed under either:

- MIT license
- Apache License, Version 2.0
