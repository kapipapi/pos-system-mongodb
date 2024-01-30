use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};
use crate::models::CollectionName;

const TABLES_COLL_NAME: &str = "tables";

pub type TableId = Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewTable {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub level: i32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Table {
    pub _id: TableId,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub level: i32,
}

impl CollectionName for Table {
    fn collection_name() -> &'static str {
        TABLES_COLL_NAME
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TableAPI {
    pub _id: TableId,
    pub name: String,
}

impl CollectionName for TableAPI {
    fn collection_name() -> &'static str {
        TABLES_COLL_NAME
    }
}