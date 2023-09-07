use spacetimedb::{spacetimedb, Identity};

#[spacetimedb(table)]
pub struct User {
    #[primarykey]
    pub identity: Identity,
    pub name: Option<String>,
    pub online: bool,
}
