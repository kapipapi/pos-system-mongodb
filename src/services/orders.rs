use actix_web::{delete, get, HttpResponse, post, web};
use mongodb::{bson};
use mongodb::bson::{doc, Uuid};
use crate::models::orders::{NewOrder, Order, OrderId};
use crate::models::products::{AddProductQuery};
use crate::models::tables::TableId;
use crate::models::waiters::WaiterId;
use crate::repo::repository::Repository;
use crate::services::error::ServiceError;

#[get("/orders")]
pub(crate) async fn get_all_orders(repo: web::Data<Repository>) -> Result<HttpResponse, ServiceError> {
    let result = repo.query_all_orders_api().await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/orders")]
pub(crate) async fn add_order(repo: web::Data<Repository>, data: web::Json<NewOrder>) -> Result<HttpResponse, ServiceError> {
    let new_order = Order {
        _id: OrderId::new(),
        waiter_id: data.waiter_id,
        table_id: data.table_id,
        products: vec![],
        created_at: bson::DateTime::now(),
    };

    repo.insert_one::<Order>(new_order.clone()).await?;

    Ok(HttpResponse::Ok().json(new_order))
}

#[get("/orders/{id}")]
pub(crate) async fn get_order(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = OrderId::parse_str(&id.into_inner()).unwrap();

    let result = repo.query_order_api(&id).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/orders/{id}/add-product")]
pub(crate) async fn add_product_to_order(repo: web::Data<Repository>, id: web::Path<String>, data: web::Json<AddProductQuery>) -> Result<HttpResponse, ServiceError> {
    let id = OrderId::parse_str(&id.into_inner()).unwrap();
    let add_product_query = data.into_inner();

    let result = repo.order_add_product(&id, &add_product_query.product_id).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/orders/{id}/remove-product")]
pub(crate) async fn remove_product_from_order(repo: web::Data<Repository>, id: web::Path<String>, data: web::Json<AddProductQuery>) -> Result<HttpResponse, ServiceError> {
    let id = OrderId::parse_str(&id.into_inner()).unwrap();
    let add_product_query = data.into_inner();

    let result = repo.order_remove_product(&id, &add_product_query.product_id).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/orders/waiter/{id}")]
pub(crate) async fn get_orders_by_waiter(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = WaiterId::parse_str(&id.into_inner()).unwrap();

    let result = repo.query_orders_by_waiter(&id).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/orders/table/{id}")]
pub(crate) async fn get_orders_by_table(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = TableId::parse_str(&id.into_inner()).unwrap();

    let result = repo.query_orders_by_table(&id).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/orders/{id}/check-empty")]
pub(crate) async fn check_empty_order(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = TableId::parse_str(&id.into_inner()).unwrap();

    let result = repo.query_order_api(&id).await?;

    if result.products.len() > 0 {
        return Ok(HttpResponse::Ok().json(false));
    }

    repo.delete_one::<Order>(&id).await?;

    Ok(HttpResponse::Ok().json(true))
}