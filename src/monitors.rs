use serenity::prelude::Context;
use serenity::model::channel::Message;

mod anilist;

pub fn run_monitors(ctx: Context, message: Message) {
    if !message.author.bot {
        anilist::anilist_monitor(ctx, message);
    }
}