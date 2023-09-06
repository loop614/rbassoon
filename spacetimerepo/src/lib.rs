use spacetimedb::{spacetimedb, ReducerContext, Identity, Timestamp};
mod table;
mod reducer;
mod validator;

use crate::table::user::User;

#[spacetimedb(connect)]
pub fn identity_connected(ctx: ReducerContext) {
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(&ctx.sender, User { online: true, ..user });
        return;
    }

    User::insert(User {
        name: None,
        identity: ctx.sender,
        online: true,
    }).unwrap();
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(ctx: ReducerContext) {
    if let Some(user) = User::filter_by_identity(&ctx.sender) {
        User::update_by_identity(&ctx.sender, User { online: false, ..user });
    }

    log::warn!("Disconnect event for unknown user with identity {:?}", ctx.sender);
}
