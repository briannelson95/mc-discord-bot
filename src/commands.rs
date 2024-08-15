use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use std::time::Duration;
use elytra_ping::{self, ping_or_timeout};

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply(format!("Pong!")).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn status(
    ctx: Context<'_>, 
    #[description = "Server status"] user: Option<serenity::User>,
) -> Result<(), Error> {
    match elytra_ping::ping_or_timeout(
        ("mc.hypixel.net".to_string(), 25565),
        Duration::from_secs(1),
    ).await {
        Ok((ping_info, latency)) => {
            println!("{:#?}", ping_info);  // Print out the ping_info structure
            ctx.say("Ping info received! Check console for details.").await?;
            Ok(())
        }
        Err(e) => {
            ctx.say(format!("Failed to ping server: {:?}", e)).await?;
            Err(Box::new(e))
        }
    }
}
