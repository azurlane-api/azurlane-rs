# azurlane-rs
Wrapper for the unofficial azur lane json api in Rust


## Example
```rust
use azurlane_rs::{AzurLaneRequester, Order};
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