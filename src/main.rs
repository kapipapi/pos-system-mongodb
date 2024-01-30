mod models;
#[cfg(test)]
mod tests;
mod services;
mod repo;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use actix_web::http::header;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::repo::repository::Repository;
use crate::services::auth::validator;
use crate::services::orders::{add_order, add_product_to_order, get_all_orders, get_order, get_orders_by_waiter, remove_product_from_order};
use crate::services::products::{add_product, get_all_products, get_product};
use crate::services::tables::{add_table, get_all_tables, get_table};
use crate::services::waiters::{add_waiter, get_all_waiters, get_waiter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = Repository::connect().await;

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .max_age(3600);
        let logger = middleware::Logger::default();

        App::new()
            .wrap(auth)
            .wrap(cors)
            .app_data(web::Data::new(repo.clone()))
            .wrap(logger)
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
            .service(get_all_tables)
            .service(get_table)
            .service(add_table)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
