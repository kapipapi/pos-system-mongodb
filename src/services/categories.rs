use actix_web::{get, HttpResponse, post, web};
use mongodb::bson::{doc};
use crate::models::categories::{Category, CategoryId, NewCategory};
use crate::repo::repository::Repository;
use crate::services::error::ServiceError;

#[get("/categories")]
pub(crate) async fn get_all_categories(repo: web::Data<Repository>) -> Result<HttpResponse, ServiceError> {
    let result = repo.query_all::<Category>().await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/categories")]
pub(crate) async fn add_category(repo: web::Data<Repository>, data: web::Json<NewCategory>) -> Result<HttpResponse, ServiceError> {
    let data = data.into_inner();

    let new_category = Category {
        _id: CategoryId::new(),
        name: data.name,
        icon: data.icon,
        color: data.color,
    };

    repo.insert_one::<Category>(new_category.clone()).await.map_err(|err| ServiceError::InternalError(err.to_string()))?;

    Ok(HttpResponse::Ok().json(new_category))
}

#[get("/categories/{id}")]
pub(crate) async fn get_category(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = CategoryId::parse_str(&id.into_inner()).unwrap();

    let collection = repo.get_collection::<Category>();

    let result = collection.find_one(doc! { "_id": id }, None).await.map_err(|err| ServiceError::InternalError(err.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}