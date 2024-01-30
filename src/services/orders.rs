use actix_web::{get, HttpResponse, post, web};
use mongodb::{bson};
use mongodb::bson::{doc};
use crate::models::orders::{NewOrder, Order, OrderId};
use crate::models::products::{AddProductQuery};
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

#[post("/orders/{id}/products")]
pub(crate) async fn add_product_to_order(repo: web::Data<Repository>, id: web::Path<String>, data: web::Json<AddProductQuery>) -> Result<HttpResponse, ServiceError> {
    let id = OrderId::parse_str(&id.into_inner()).unwrap();
    let add_product_query = data.into_inner();

    let collection = repo.get_collection::<Order>();
    collection.find_one_and_update(
        doc! { "_id": id },
        doc! { "$push": { "products": add_product_query.product_id } },
        None,
    ).await.map_err(|err| ServiceError::InternalError(err.to_string()))?;

    let result = repo.query_order_api(&id).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/orders/waiter/{id}")]
pub(crate) async fn get_orders_by_waiter(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = WaiterId::parse_str(&id.into_inner()).unwrap();

    let result = repo.query_orders_by_waiter(&id).await?;

    Ok(HttpResponse::Ok().json(result))
}