use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};
use crate::models::CollectionName;
use crate::models::products::{ProductInOrder, ProductId, ProductIdWithQuantity};
use crate::models::tables::{TableInOrder, TableId};
use crate::models::waiters::{WaiterInOrder, WaiterId};

const ORDERS_COLL_NAME: &str = "orders";

pub type OrderId = Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewOrder {
    pub waiter_id: WaiterId,
    pub table_id: TableId,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Order {
    pub _id: OrderId,
    pub waiter_id: WaiterId,
    pub table_id: TableId,
    pub products: Vec<ProductIdWithQuantity>,
    pub created_at: DateTime,
}

impl CollectionName for Order {
    fn collection_name() -> &'static str {
        ORDERS_COLL_NAME
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OrderAPI {
    pub _id: OrderId,
    pub waiter: WaiterInOrder,
    pub table: TableInOrder,
    pub products: Vec<ProductInOrder>,
    pub sum: f64,
    pub created_at: DateTime,
}

