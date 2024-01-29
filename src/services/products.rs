use actix_web::{get, HttpResponse, post, web};
use mongodb::bson::{doc};
use crate::models::products::{NewProduct, Product, ProductId};
use crate::repo::repository::Repository;

#[get("/products")]
pub(crate) async fn get_all_products(repo: web::Data<Repository>) -> HttpResponse {
    let result = repo.query_all::<Product>().await;
    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/products")]
pub(crate) async fn add_product(repo: web::Data<Repository>, data: web::Json<NewProduct>) -> HttpResponse {
    let data = data.into_inner();

    let new_product = Product {
        _id: ProductId::new(),
        name: data.name,
        price: data.price,
    };

    let result = repo.insert_one::<Product>(new_product.clone()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(new_product),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/products/{id}")]
pub(crate) async fn get_product(repo: web::Data<Repository>, id: web::Path<String>) -> HttpResponse {
    let id = ProductId::parse_str(&id.into_inner()).unwrap();

    let collection = repo.get_collection::<Product>();

    let result = collection.find_one(doc! { "_id": id }, None).await;
    match result {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body(format!("No product found with id: {id}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}