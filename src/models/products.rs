use mongodb::bson::{Uuid};
use serde::{Deserialize, Serialize};
use crate::models::CollectionName;

const PRODUCTS_COLL_NAME: &str = "products";

pub type ProductId = Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewProduct {
    pub name: String,
    pub price: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Product {
    pub _id: ProductId,
    pub name: String,
    pub price: f64,
}

impl CollectionName for Product {
    fn collection_name() -> &'static str {
        PRODUCTS_COLL_NAME
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddProductQuery {
    pub product_id: ProductId,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ProductAPI {
    pub _id: ProductId,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}