use spacetimedb::{spacetimedb, ReducerContext};

use crate::validator::user as validator;
use crate::table::user::User;
#[spacetimedb(reducer)]
pub fn say_hello() {
    for user in User::iter() {
        log::info!("Hello, {:?}!", user.name);
    }
    log::info!("Hello, World!");
}

#[spacetimedb(reducer)]
pub fn set_name(ctx: ReducerContext, name: String) -> Result<(), String> {
    let name = validator::validate_name(name)?;
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(&ctx.sender, User { name: Some(name), ..user });
        return Ok(())
    }

    Err("Cannot set name for unknown user".to_string())
}
