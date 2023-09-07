use spacetimedb::{spacetimedb, ReducerContext};

use crate::validator::message as validator;
use crate::table::message::Message;

#[spacetimedb(reducer)]
pub fn send_message(ctx: ReducerContext, text: String) -> Result<(), String> {
    let text = validator::validate_message(text)?;
    log::info!("{}", text);
    Message::insert(Message {
        sender: ctx.sender,
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}
