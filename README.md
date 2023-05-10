# Discord Price Bot

An experimental project that uses Rust libraries to create a Discord bot to display crypto token prices from https://jup.ag/
The bot displays the USD equivalent of a given token.

Required env variables:

- `DISCORD_TOKEN`: [Discord](https://discord.com/developers/applications/) bot token
- `CRYPTO_TOKEN`: Pubkey of a token from Jup - https://jup.ag/
- `CRYPTO_TOKEN_NAME`: Name of token i.e. BTC, ETH, SOL
- `GUILD_ID`: ID of the discord channel

## Important crates used:
- `serenity`: Discord API in Rust
  - https://docs.shuttle.rs/resources/shuttle-secrets
- `jup`: Price API
  - https://crates.io/crates/jup-ag/
- `currency_rs`: Formats float to currency
  - https://docs.rs/currency_rs/latest/currency_rs/#
- `shuttle`: Rust serverless deployment
  - https://docs.rs/shuttle-service/latest/shuttle_service/

## Deployment
```
# Initialize `shuttle` configurations
# Prjects will be hosted at ${project_name}.shuttleapp.rs
cargo shuttle init --serenity

# Start
cargo shuttle project start

# Run the program locally 
cargo shuttle run

# Deploy and run the program 
cargo shuttle deploy
```

