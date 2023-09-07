// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SayHelloArgs {}

impl Reducer for SayHelloArgs {
    const REDUCER_NAME: &'static str = "say_hello";
}

#[allow(unused)]
pub fn say_hello() {
    SayHelloArgs {}.invoke();
}

#[allow(unused)]
pub fn on_say_hello(
    mut __callback: impl FnMut(&Identity, &Status) + Send + 'static,
) -> ReducerCallbackId<SayHelloArgs> {
    SayHelloArgs::on_reducer(move |__identity, __status, __args| {
        let SayHelloArgs {} = __args;
        __callback(__identity, __status);
    })
}

#[allow(unused)]
pub fn once_on_say_hello(
    __callback: impl FnOnce(&Identity, &Status) + Send + 'static,
) -> ReducerCallbackId<SayHelloArgs> {
    SayHelloArgs::once_on_reducer(move |__identity, __status, __args| {
        let SayHelloArgs {} = __args;
        __callback(__identity, __status);
    })
}

#[allow(unused)]
pub fn remove_on_say_hello(id: ReducerCallbackId<SayHelloArgs>) {
    SayHelloArgs::remove_on_reducer(id);
}
