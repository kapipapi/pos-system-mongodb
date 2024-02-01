use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};
use crate::models::CollectionName;

const WAITERS_COLL_NAME: &str = "waiters";

pub type WaiterId = Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewWaiter {
    pub name: String,
    pub code: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Waiter {
    pub _id: WaiterId,
    pub name: String,
    pub code: String,
}

impl CollectionName for Waiter {
    fn collection_name() -> &'static str {
        WAITERS_COLL_NAME
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaiterInOrder {
    pub _id: WaiterId,
    pub name: String,
}

impl CollectionName for WaiterInOrder {
    fn collection_name() -> &'static str {
        WAITERS_COLL_NAME
    }
}