use actix_web::{get, HttpResponse, post, web};
use mongodb::bson::{doc};
use crate::models::products::{NewProduct, Product, ProductId};
use crate::repo::repository::Repository;
use crate::services::error::ServiceError;

#[get("/products")]
pub(crate) async fn get_all_products(repo: web::Data<Repository>) -> Result<HttpResponse, ServiceError> {
    let result = repo.query_all::<Product>().await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/products")]
pub(crate) async fn add_product(repo: web::Data<Repository>, data: web::Json<NewProduct>) -> Result<HttpResponse, ServiceError> {
    let data = data.into_inner();

    let new_product = Product {
        _id: ProductId::new(),
        name: data.name,
        price: data.price,
    };

    repo.insert_one::<Product>(new_product.clone()).await.map_err(|err| ServiceError::InternalError(err.to_string()))?;

    Ok(HttpResponse::Ok().json(new_product))
}

#[get("/products/{id}")]
pub(crate) async fn get_product(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = ProductId::parse_str(&id.into_inner()).unwrap();

    let collection = repo.get_collection::<Product>();

    let result = collection.find_one(doc! { "_id": id }, None).await.map_err(|err| ServiceError::InternalError(err.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}