use spacetimedb::{spacetimedb, ReducerContext, Identity, Timestamp};

#[spacetimedb(table)]
pub struct User {
    #[primarykey]
    identity: Identity,
    name: Option<String>,
    online: bool,
}

#[spacetimedb(table)]
pub struct Message {
    sender: Identity,
    sent: Timestamp,
    text: String,
}

#[spacetimedb(reducer)]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(&ctx.sender, User { name: Some(name), ..user });
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

/// Takes a name and checks if it's acceptable as a user's name.
fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

#[spacetimedb(reducer)]
pub fn say_hello() {
    for user in User::iter() {
        log::info!("Hello, {:?}!", user.name);
    }
    log::info!("Hello, World!");
}

#[spacetimedb(reducer)]
pub fn send_message(ctx: ReducerContext, text: String) -> Result<(), String> {
    let text = validate_message(text)?;
    log::info!("{}", text);
    Message::insert(Message {
        sender: ctx.sender,
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

/// Takes a message's text and checks if it's acceptable to send.
fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[spacetimedb(connect)]
// Called when a client connects to the SpacetimeDB
pub fn identity_connected(ctx: ReducerContext) {
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        // If this is a returning user, i.e. we already have a `User` with this `Identity`,
        // set `online: true`, but leave `name` and `identity` unchanged.
        User::update_by_identity(&ctx.sender, User { online: true, ..user });
    } else {
        // If this is a new user, create a `User` row for the `Identity`,
        // which is online, but hasn't set a name.
        User::insert(User {
            name: None,
            identity: ctx.sender,
            online: true,
        }).unwrap();
    }
}

#[spacetimedb(disconnect)]
// Called when a client disconnects from SpacetimeDB
pub fn identity_disconnected(ctx: ReducerContext) {
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(&ctx.sender, User { online: false, ..user });
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!("Disconnect event for unknown user with identity {:?}", ctx.sender);
    }
}
