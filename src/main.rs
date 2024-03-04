mod models;
#[cfg(test)]
mod tests;
mod services;
mod repo;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::repo::repository::Repository;
use crate::services::auth::validator;
use crate::services::categories::{add_category, get_all_categories, get_category};
use crate::services::orders::{add_order, add_product_to_order, check_empty_order, get_all_orders, get_order, get_orders_by_table, get_orders_by_waiter, remove_product_from_order};
use crate::services::products::{add_product, get_all_products, get_product};
use crate::services::tables::{add_table, get_all_tables, get_table};
use crate::services::waiters::{add_waiter, delete_waiter, get_all_waiters, get_waiter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = Repository::connect().await;

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(auth)
            .wrap(cors)
            .app_data(web::Data::new(repo.clone()))
            .service(add_waiter)
            .service(get_waiter)
            .service(get_all_waiters)
            .service(delete_waiter)
            .service(add_order)
            .service(get_order)
            .service(get_all_orders)
            .service(get_orders_by_waiter)
            .service(get_orders_by_table)
            .service(add_product_to_order)
            .service(remove_product_from_order)
            .service(check_empty_order)
            .service(add_product)
            .service(get_product)
            .service(get_all_products)
            .service(get_all_tables)
            .service(get_table)
            .service(add_table)
            .service(get_all_categories)
            .service(add_category)
            .service(get_category)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
