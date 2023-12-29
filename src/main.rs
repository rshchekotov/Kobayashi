mod event;

// use mochi;
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

struct Data {
    bot_owner: Mutex<Option<serenity::User>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    /* read potential .env files */
    dotenv::dotenv().ok();

    let _framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event::event_handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                )),
                ignore_bots: true,
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![age()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("Missing Discord token"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    bot_owner: Mutex::from(None),
                })
            })
        });
    // framework.run().await.unwrap(); // 受付
}
