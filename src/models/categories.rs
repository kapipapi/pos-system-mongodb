use mongodb::bson::{Uuid};
use serde::{Deserialize, Serialize};
use crate::models::CollectionName;

const ORDERS_COLL_NAME: &str = "categories";

pub type CategoryId = Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewCategory {
    pub name: String,
    pub icon: String,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Category {
    pub _id: CategoryId,
    pub name: String,
    pub icon: String,
    pub color: String,
}

impl CollectionName for Category {
    fn collection_name() -> &'static str {
        ORDERS_COLL_NAME
    }
}

