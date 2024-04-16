use std::error::Error;
use std::ops::BitOr;
use teloxide::adaptors::Trace;
use teloxide::adaptors::trace;
use teloxide::adaptors::throttle;
use teloxide::adaptors::throttle::*;
use teloxide::prelude::*;
use teloxide::dispatching::*;
use teloxide::types::{InputFile, Me};

type BotT = Trace<Throttle<Bot>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot: BotT = Bot::from_env()
        .throttle(Limits::default())
        .trace(trace::Settings::all());


    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}

async fn message_handler(
    bot: BotT,
    msg: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    //bot.send_sticker(msg.chat.id, InputFile::file_id("AQADFQADwDZPE3I")).await?;
    //bot.send_dice(msg.chat.id).await?;

    bot.send_message(msg.chat.id, msg.text().unwrap_or_default()).await?;
    Ok(())
}
