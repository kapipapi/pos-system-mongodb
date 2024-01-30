use std::collections::HashMap;
use std::hash::Hash;
use futures::TryStreamExt;
use mongodb::bson::doc;
use crate::models::orders::{Order, OrderAPI, OrderId};
use crate::models::products::{Product, ProductAPI};
use crate::models::waiters::{WaiterAPI, WaiterId};
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

        let waiter = self.query_one::<WaiterAPI>(&order.waiter_id).await?;

        let products_frequency = count_frequency(order.products.clone());
        let products = self.query_many::<Product>(&order.products).await?;

        let mut sum: f64 = 0.0;
        let products = products.iter().map(|product| {
            let quantity = products_frequency[&product._id] as u32;
            sum += product.price * quantity as f64;
            ProductAPI {
                _id: product._id,
                name: product.name.clone(),
                price: product.price,
                quantity,
            }
        }).collect::<Vec<ProductAPI>>();

        Ok(
            OrderAPI {
                _id: order._id,
                waiter,
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