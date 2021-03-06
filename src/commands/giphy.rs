use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::core::store::PaginationKind;
use crate::menu;
use crate::menu::builders;
use crate::models::giphy::*;

pub fn query(query: String) -> GiphyResponse {
    let giphy_key = dotenv::var("GIPHY_API_KEY").expect("giphy_api_token");
    let client = reqwest::blocking::Client::new();

    let endpoint = if !query.is_empty() {
        format!("search?q={}&", query)
    } else {
        "trending?".to_owned()
    };

    let request = format!(
        "http://api.giphy.com/v1/gifs/{}api_key={}&fmt=json",
        endpoint, giphy_key
    );
    let res = client.get(request.as_str()).send().expect("response");
    let response: GiphyResponse = res.json().expect("json");

    response
}

#[command]
#[aliases("gif")]
#[usage = "[keyword]"]
fn giphy(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let keyword = args.message().to_string();
    let results = query(keyword.clone()).data;

    if results.is_empty() {
        return Err(CommandError(format!("No gif was found for `{}`.", keyword)));
    }

    let gif: &Giphy = &results[0];
    let sending = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.clone_from(&builders::giphy_embed_builder(
                gif,
                format!("Page: {}/{} | ", 1, results.len()),
            ));

            e
        })
        .reactions(menu::reactions::default())
    });

    if let Ok(sending_msg) = sending {
        menu::new_pagination(
            context,
            sending_msg.id,
            message.author.id,
            PaginationKind::Giphy,
            menu::utils::serialize_entries(results),
        )
    }

    Ok(())
}
