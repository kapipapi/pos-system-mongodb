use actix_web::{get, HttpResponse, post, web};
use mongodb::{bson};
use mongodb::bson::{doc};
use crate::models::orders::{NewOrder, Order, OrderId};
use crate::models::products::{AddProductQuery};
use crate::repo::repository::Repository;
use crate::repo::error::RepoError;

#[get("/orders")]
pub(crate) async fn get_all_orders(repo: web::Data<Repository>) -> HttpResponse {
    let result = repo.query_all::<Order>().await;
    match result {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/orders")]
pub(crate) async fn add_order(repo: web::Data<Repository>, data: web::Json<NewOrder>) -> HttpResponse {
    let new_order = Order {
        _id: OrderId::new(),
        waiter_id: data.waiter_id,
        products: vec![],
        created_at: bson::DateTime::now(),
    };

    let result = repo.insert_one::<Order>(new_order.clone()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(new_order),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/orders/{id}")]
pub(crate) async fn get_order(repo: web::Data<Repository>, id: web::Path<String>) -> HttpResponse {
    let id = OrderId::parse_str(&id.into_inner()).unwrap();

    let result = repo.query_order_api(&id).await;

    match result {
        Ok(order_api) => HttpResponse::Ok().json(order_api),
        Err(err) => match err {
            RepoError::NotFound(msg) => HttpResponse::NotFound().body(msg),
            RepoError::InternalServerError(msg) => HttpResponse::InternalServerError().body(msg),
        }
    }
}

#[post("/orders/{id}/products")]
pub(crate) async fn add_product_to_order(repo: web::Data<Repository>, id: web::Path<String>, data: web::Json<AddProductQuery>) -> HttpResponse {
    let id = OrderId::parse_str(&id.into_inner()).unwrap();
    let add_product_query = data.into_inner();

    let collection = repo.get_collection::<Order>();
    let result = collection.find_one_and_update(
        doc! { "_id": id },
        doc! { "$push": { "products": add_product_query.product_id } },
        None,
    ).await;
    match result {
        Ok(_) => (),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }

    let result = repo.query_order_api(&id).await;
    match result {
        Ok(order_api) => HttpResponse::Ok().json(order_api),
        Err(err) => match err {
            RepoError::NotFound(msg) => HttpResponse::NotFound().body(msg),
            RepoError::InternalServerError(msg) => HttpResponse::InternalServerError().body(msg),
        }
    }
}