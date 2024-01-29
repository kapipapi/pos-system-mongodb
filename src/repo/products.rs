use std::collections::HashMap;
use std::hash::Hash;
use crate::models::orders::{Order, OrderAPI, OrderId};
use crate::models::products::{Product, ProductAPI};
use crate::models::waiters::WaiterAPI;
use crate::repo::error::RepoError;
use crate::repo::repository::Repository;

impl Repository {
    pub async fn query_order_api(&self, id: &OrderId) -> Result<OrderAPI, RepoError> {
        let result = self.query_one::<Order>(&id).await;
        let order = match result {
            Ok(Some(order)) => order,
            Ok(None) => return Err(RepoError::NotFound(format!("with id: {id}"))),
            Err(err) => return Err(RepoError::InternalServerError(err.to_string())),
        };

        let waiter = self.query_one::<WaiterAPI>(&order.waiter_id).await;
        let waiter = match waiter {
            Ok(Some(waiter)) => waiter,
            Ok(None) => return Err(RepoError::NotFound(format!("with id: {id}"))),
            Err(err) => return Err(RepoError::InternalServerError(err.to_string())),
        };

        let products_frequency = count_frequency(order.products.clone());
        let products = self.query_many::<Product>(&order.products).await
            .map_err(|err| RepoError::InternalServerError(err.to_string()))?;

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