mod models;
#[cfg(test)]
mod tests;
mod services;
mod repo;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use crate::repo::repository::Repository;
use crate::services::orders::{add_order, add_product_to_order, get_all_orders, get_order, get_orders_by_waiter, remove_product_from_order};
use crate::services::products::{add_product, get_all_products, get_product};
use crate::services::waiters::{add_waiter, get_all_waiters, get_waiter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = Repository::connect().await;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(repo.clone()))
            .service(add_waiter)
            .service(get_waiter)
            .service(get_all_waiters)
            .service(add_order)
            .service(get_order)
            .service(get_all_orders)
            .service(get_orders_by_waiter)
            .service(add_product_to_order)
            .service(remove_product_from_order)
            .service(add_product)
            .service(get_product)
            .service(get_all_products)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
