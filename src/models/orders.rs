use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};
use crate::models::CollectionName;
use crate::models::products::{ProductAPI, ProductId};
use crate::models::waiters::{WaiterAPI, WaiterId};

const ORDERS_COLL_NAME: &str = "orders";

pub type OrderId = Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewOrder {
    pub waiter_id: WaiterId,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Order {
    pub _id: OrderId,
    pub waiter_id: WaiterId,
    pub products: Vec<ProductId>,
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
    pub waiter: WaiterAPI,
    pub products: Vec<ProductAPI>,
    pub sum: f64,
    pub created_at: DateTime,
}

