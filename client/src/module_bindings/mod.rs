// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use spacetimedb_sdk::callbacks::{DbCallbacks, ReducerCallbacks};
use spacetimedb_sdk::client_api_messages::{Event, TableUpdate};
use spacetimedb_sdk::client_cache::{ClientCache, RowCallbackReminders};
use spacetimedb_sdk::global_connection::with_connection_mut;
use spacetimedb_sdk::identity::Credentials;
use spacetimedb_sdk::reducer::AnyReducerEvent;
#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
};
use std::sync::Arc;

pub mod message;
pub mod say_hello_reducer;
pub mod send_message_reducer;
pub mod set_name_reducer;
pub mod user;

pub use message::*;
pub use say_hello_reducer::*;
pub use send_message_reducer::*;
pub use set_name_reducer::*;
pub use user::*;

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ReducerEvent {
    SayHello(say_hello_reducer::SayHelloArgs),
    SendMessage(send_message_reducer::SendMessageArgs),
    SetName(set_name_reducer::SetNameArgs),
}

#[allow(unused)]
fn handle_table_update(
    table_update: TableUpdate,
    client_cache: &mut ClientCache,
    callbacks: &mut RowCallbackReminders,
) {
    let table_name = &table_update.table_name[..];
    match table_name {
        "Message" => client_cache
            .handle_table_update_no_primary_key::<message::Message>(callbacks, table_update),
        "User" => {
            client_cache.handle_table_update_with_primary_key::<user::User>(callbacks, table_update)
        }
        _ => spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name),
    }
}

#[allow(unused)]
fn invoke_row_callbacks(
    reminders: &mut RowCallbackReminders,
    worker: &mut DbCallbacks,
    reducer_event: Option<Arc<AnyReducerEvent>>,
    state: &Arc<ClientCache>,
) {
    reminders.invoke_callbacks::<message::Message>(worker, &reducer_event, state);
    reminders.invoke_callbacks::<user::User>(worker, &reducer_event, state);
}

#[allow(unused)]
fn handle_resubscribe(
    new_subs: TableUpdate,
    client_cache: &mut ClientCache,
    callbacks: &mut RowCallbackReminders,
) {
    let table_name = &new_subs.table_name[..];
    match table_name {
        "Message" => {
            client_cache.handle_resubscribe_for_type::<message::Message>(callbacks, new_subs)
        }
        "User" => client_cache.handle_resubscribe_for_type::<user::User>(callbacks, new_subs),
        _ => spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name),
    }
}

#[allow(unused)]
fn handle_event(
    event: Event,
    reducer_callbacks: &mut ReducerCallbacks,
    state: Arc<ClientCache>,
) -> Option<Arc<AnyReducerEvent>> {
    let Some(function_call) = &event.function_call else {
				spacetimedb_sdk::log::warn!("Received Event with None function_call"); return None;
};
    match &function_call.reducer[..] {
        "say_hello" => reducer_callbacks
            .handle_event_of_type::<say_hello_reducer::SayHelloArgs, ReducerEvent>(
                event,
                state,
                ReducerEvent::SayHello,
            ),
        "send_message" => reducer_callbacks
            .handle_event_of_type::<send_message_reducer::SendMessageArgs, ReducerEvent>(
                event,
                state,
                ReducerEvent::SendMessage,
            ),
        "set_name" => reducer_callbacks
            .handle_event_of_type::<set_name_reducer::SetNameArgs, ReducerEvent>(
                event,
                state,
                ReducerEvent::SetName,
            ),
        unknown => {
            spacetimedb_sdk::log::error!("Event on an unknown reducer: {:?}", unknown);
            None
        }
    }
}

/// Connect to a database named `db_name` accessible over the internet at the URI `spacetimedb_uri`.
///
/// If `credentials` are supplied, they will be passed to the new connection to
/// identify and authenticate the user. Otherwise, a set of `Credentials` will be
/// generated by the server.
pub fn connect<IntoUri>(
    spacetimedb_uri: IntoUri,
    db_name: &str,
    credentials: Option<Credentials>,
) -> Result<()>
where
    IntoUri: TryInto<spacetimedb_sdk::http::Uri>,
    <IntoUri as TryInto<spacetimedb_sdk::http::Uri>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    with_connection_mut(|connection| {
        connection.connect(
            spacetimedb_uri,
            db_name,
            credentials,
            handle_table_update,
            handle_resubscribe,
            invoke_row_callbacks,
            handle_event,
        )?;
        Ok(())
    })
}
