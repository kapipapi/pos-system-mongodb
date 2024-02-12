use std::collections::HashMap;
use std::hash::Hash;
use futures::TryStreamExt;
use mongodb::bson::{to_bson, doc, Uuid};
use crate::models::categories::{Category, CategoryId};
use crate::models::orders::{Order, OrderAPI, OrderId};
use crate::models::products::{Product, ProductInOrder, ProductId, ProductIdWithQuantity};
use crate::models::tables::{TableId, TableInOrder};
use crate::models::waiters::{WaiterInOrder, WaiterId};
use crate::repo::error::RepoError;
use crate::repo::repository::Repository;

impl Repository {
    pub async fn query_all_orders_api(&self) -> Result<Vec<OrderAPI>, RepoError> {
        let mut cursor = self.get_collection::<Order>().find(
            None,
            None,
        ).await?;

        let mut results: Vec<OrderAPI> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            results.push(self.query_order_api(&result._id).await?);
        }

        Ok(results)
    }

    pub async fn query_order_api(&self, id: &OrderId) -> Result<OrderAPI, RepoError> {
        let order = self.query_one::<Order>(&id).await?;

        let waiter = self.query_one::<WaiterInOrder>(&order.waiter_id).await?;
        let table = self.query_one::<TableInOrder>(&order.table_id).await?;
        let categories = self.query_all::<Category>().await?;

        let products_ids = order.products.iter().map(|product| product._id as Uuid).collect::<Vec<Uuid>>();

        let products = self
            .query_many::<Product>(&products_ids)
            .await?
            .into_iter()
            .map(|product| {
                let quantity = order.products.iter().find(|p| p._id == product._id).unwrap().quantity;
                let category = categories.iter().find(|c| c._id == product.category_id).unwrap();
                ProductInOrder {
                    _id: product._id,
                    name: product.name.clone(),
                    price: product.price,
                    category: category.clone(),
                    quantity,
                }
            })
            .collect::<Vec<ProductInOrder>>();

        let sum = products.iter().fold(0.0, |acc, product| acc + product.price * product.quantity);

        Ok(
            OrderAPI {
                _id: order._id,
                waiter,
                table,
                products,
                sum,
                created_at: order.created_at,
            }
        )
    }

    pub async fn query_orders_by_waiter(&self, id: &WaiterId) -> Result<Vec<OrderAPI>, RepoError> {
        let mut cursor = self.get_collection::<Order>().find(
            Some(doc! { "waiter_id": id }),
            None,
        ).await?;


        let mut results: Vec<OrderAPI> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            results.push(self.query_order_api(&result._id).await?);
        }

        Ok(results)
    }

    pub async fn query_orders_by_table(&self, id: &TableId) -> Result<Vec<OrderAPI>, RepoError> {
        let mut cursor = self.get_collection::<Order>().find(
            Some(doc! { "table_id": id }),
            None,
        ).await?;

        let mut results: Vec<OrderAPI> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            results.push(self.query_order_api(&result._id).await?);
        }

        Ok(results)
    }

    pub async fn order_add_product(&self, id: &OrderId, product_id: &ProductId) -> Result<OrderAPI, RepoError> {
        let collection = self.get_collection::<Order>();

        let filter = doc! { "_id": id, "products._id": product_id };

        let result = collection.find_one(
            filter.clone(),
            None,
        ).await?;

        if result.is_some() {
            log::info!("Product already exists in order, incrementing quantity");
            collection.update_one(
                filter,
                doc! {
                        "$inc": {
                            "products.$.quantity": 1
                        }
                    },
                None,
            ).await?;
        } else {
            log::info!("Product does not exist in order, adding it");

            let product = ProductIdWithQuantity {
                _id: *product_id,
                quantity: 1.0,
            };

            let product_bson = to_bson(&product).map_err(|err| RepoError::BsonSerializationError(err))?;

            collection.find_one_and_update(
                doc! { "_id": id },
                doc! { "$push": { "products": product_bson } },
                None,
            ).await?;
        }

        self.query_order_api(&id).await
    }

    pub async fn order_remove_product(&self, id: &OrderId, product_id: &ProductId) -> Result<OrderAPI, RepoError> {
        let collection = self.get_collection::<Order>();

        let filter = doc! { "_id": id, "products._id": product_id };

        collection.find_one_and_update(
            filter.clone(),
            doc! {
                        "$inc": {
                            "products.$.quantity": -1
                        }
                    },
            None,
        ).await?;

        collection.find_one_and_update(
            filter,
            doc! {
                        "$pull": {
                            "products": {
                                "quantity": { "$lte": 0 }
                            }
                        }
                    },
            None,
        ).await?;

        self.query_order_api(&id).await
    }
}


fn count_frequency<T>(arr: Vec<T>) -> HashMap<T, i32>
    where T: Eq + Hash
{
    let mut frequency: HashMap<T, i32> = HashMap::new();

    for num in arr {
        *frequency.entry(num).or_insert(0) += 1;
    }

    frequency
}