[![v1.0.0](https://img.shields.io/badge/crates.io-v1.0.0-blue.svg)](https://crates.io/crates/azurlane)

# azurlane-rs
Wrapper for the unofficial azur lane json api in Rust

## Installation
```toml
[dependencies]
azurlane = "1.0"
```

## Example
```rust
use azurlane::{AzurLaneRequester, Order};
use reqwest::Client;

fn main() {
    let client = Client::new();
    
    let _ = match client.get_ships(Order::RARITY, "Super Rare") {
        Ok(response) => {
            for i in 0..response.ships.len() {
                println!("[{}]: ({})", response.ships[i].id, response.ships[i].name)
            }
        }
        Err(why) => {
            panic!("{}", why)
        }
    };
    
}
```

## Support
![discord](https://discordapp.com/api/v6/guilds/240059867744698368/widget.png?style=banner2)