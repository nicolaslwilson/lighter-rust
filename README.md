# Lighter Rust SDK (WIP)

[![Crates.io](https://img.shields.io/crates/v/lighter-rust.svg)](https://crates.io/crates/lighter-rust)
[![Documentation](https://docs.rs/lighter-rust/badge.svg)](https://docs.rs/lighter-rust)
[![CI](https://github.com/yongkangc/lighter-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/yongkangc/lighter-rust/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK for [Lighter](https://lighter.xyz/) (v2)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lighter-rust = "0.1.0"
```

Or use the latest from GitHub:

```toml
[dependencies]
lighter-rust = { git = "https://github.com/yongkangc/lighter-rust" }
```

## Quick Start

```rust
use lighter_rust::{LighterClient, Config, api::account::AccountBy};

#[tokio::main]
async fn main() -> Result<()> {
    let config = LighterConfig::new()
        .with_api_key_private(YOUR_API_KEY_PRIVATE)
        .with_account_index(YOUR_ACCOUNT_INDEX)
        .with_api_key_index(YOUR_API_KEY_INDEX);
    let client = HttpClient::builder()
        .with_config(config)
        .with_account()
        .build()?;
    let account = client
        .api()
        .account()?
        .account(AccountBy::L1Address, YOUR_ACCOUNT_ADDRESS)
        .await?;
    println!("Account: {:?}", account);
    
    Ok(())
}
```

<!-- ## Examples

Check out the [examples](./examples) directory for comprehensive examples:

- [**basic_usage.rs**](./examples/basic_usage.rs) - Basic API operations and getting started
- [**websocket_example.rs**](./examples/websocket_example.rs) - Real-time WebSocket streaming
- [**trading_bot.rs**](./examples/trading_bot.rs) - Simple trading bot with SMA strategy
- [**advanced_order_management.rs**](./examples/advanced_order_management.rs) - Grid trading, stop-loss/take-profit
- [**mnemonic_wallet.rs**](./examples/mnemonic_wallet.rs) - Using mnemonic phrases and HD wallets

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example websocket_example
cargo run --example trading_bot
``` -->

## Documentation

### API Documentation & Coverage

- [**AccountApi**](./docs/AccountApi.md) - Account management operations
- [**AnnouncementApi**](./docs/AnnouncementApi.md) - Announcement operations
- [**BlockApi**](./docs/BlockApi.md) - Info related to the network blocks
- [**BridgeApi**](./docs/BridgeApi.md) - Fastbridge info
- [**CandlestickApi**](./docs/CandlestickApi.md) - Market data and OHLCV
- [**FundingApi**](./docs/FundingApi.md) - Info regarding the funding rates
- [**InfoApi**](./docs/InfoApi.md) - Other info
- [**NotificationApi**](./docs/NotificationApi.md) - Notifications operations
- [**OrderApi**](./docs/OrderApi.md) - Order placement and management
- [**ReferralApi**](./docs/ReferralApi.md) - Info related to referrals
- [**RootApi**](./docs/RootApi.md) - General platform info
- [**TransactionApi**](./docs/TransactionApi.md) - Transaction history and tracking

<!-- ## Architecture

The SDK is built with a modular architecture:

```
lighter-rust/
├── src/
│   ├── client/          # HTTP and WebSocket clients
│   ├── api/             # API endpoint implementations
│   ├── models/          # Data models and types
│   ├── signers/         # Ethereum signing (Alloy)
│   └── error.rs         # Error handling
``` -->

<!-- ## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with examples
cargo run --example basic_usage

# Build documentation
cargo doc --open
``` -->


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an unofficial SDK. Use at your own risk. Always test thoroughly before using in production.

## Support

For issues and questions:
- Open an issue on [GitHub](https://github.com/yongkangc/lighter-rust/issues)
- Check the [API documentation](https://apidocs.lighter.xyz/docs/get-started-for-programmers-1)

## Related

- [`lighter-python` SDK](https://github.com/elliottech/lighter-python)
- [`lighter-go` signing module](https://github.com/elliottech/lighter-go)
