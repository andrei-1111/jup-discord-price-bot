use shuttle_secrets::SecretStore;

use currency_rs::{Currency, CurrencyOpts};
use jup_ag;
use serenity::{
    async_trait,
    client::{Client, Context},
    model::gateway::Ready,
    model::id::GuildId,
    prelude::*,
};
use std::str::FromStr;
use {solana_sdk::pubkey, solana_sdk::pubkey::Pubkey};

struct Handler {
    token_pubkey: String,
    token_name: String,
    guild_id: String,
}

fn usd_format(value: f64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);

    Currency::new_float(value, Some(otp))
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let usdc = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
        let token_pubkey = Pubkey::from_str(&self.token_pubkey).unwrap();
        let mut msg = format!(
            "__Connection Success__\n\n Bot Name: {} \n\n {} guilds connected;\n",
            ready.user.name,
            ready.guilds.len()
        );
        let guild_id = GuildId(self.guild_id.parse().unwrap());
        msg = format!("{}    • {}\n", msg, guild_id);
        println!("{}", msg);

        loop {
            let ui_amount = 1.;
            let data = jup_ag::price(token_pubkey, usdc, ui_amount)
                .await
                .unwrap()
                .data;
            let price = usd_format(data.price).format();
            let bot_name = format!("${}: {}", self.token_name, &price);
            if let Err(e) = ctx
                .http
                .edit_nickname(*guild_id.as_u64(), Some(&bot_name))
                .await
            {
                println!("Failed to change nickname: {:?}", e);
            } else {
                println!("Nickname successfully changed to {}", bot_name);
            }

            // Change the name every 15 secs
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Configure the client with your Discord bot token in the environment.
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN must be set.");
    let token_pubkey = secret_store
        .get("TOKEN_PUBKEY")
        .expect("TOKEN_PUBKEY must be set.");
    let token_name = secret_store
        .get("TOKEN_NAME")
        .expect("TOKEN_NAME must be set.");
    let guild_id = secret_store
        .get("GUILD_ID")
        .expect("'GUILD_ID' was not found");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(Handler {
            token_pubkey: token_pubkey,
            token_name: token_name,
            guild_id: guild_id,
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(client.into())
}
