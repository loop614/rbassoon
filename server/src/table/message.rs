use crate::{spacetimedb, Identity, Timestamp};

#[spacetimedb(table)]
pub struct Message {
    pub sender: Identity,
    pub sent: Timestamp,
    pub text: String,
}
