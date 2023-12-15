use crate::{Error, Data};

use poise::serenity_prelude::{self as serenity};
use poise::event::Event;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data
) -> Result<(), Error> {
    match event {
        // https://docs.rs/poise/latest/src/poise/event.rs.html#94-196
        Event::Ready { data_about_bot: ready } => {
            println!("Logged in as {}", ready.user.name);
            if let Ok(info) = ctx.http.get_current_application_info().await {
                let mut owner = data.bot_owner.lock().await;
                *owner = Some(info.owner.clone());
                let dm_chan = info.owner.create_dm_channel(ctx).await;
                match dm_chan {
                    Ok(chan) => {
                        let msg = chan.send_message(ctx, |m| m.content(format!("Connected & ready to serve, {}.", info.owner.name))).await;
                        if let Err(e) = msg {
                            println!("Failed to send DM to bot owner: {}", e);
                        }
                    }
                    Err(e) => {
                        println!("Failed to create DM Channel with bot owner: {}", e)
                    }
                }
            } else {
                println!("Failed to retrieve owner.")
            }
        }
        Event::Message { new_message: message } => {
            if message.content.to_lowercase().contains("Kobayashi") {
                message.reply(ctx, "You mentioned me?".to_string()).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
