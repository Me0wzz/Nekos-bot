mod api;
mod commands;
mod types;
use commands::{neko, Data};
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![neko()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
    Ok(())
}
