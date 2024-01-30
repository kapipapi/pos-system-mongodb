use actix_web::{get, post, web, HttpResponse};
use mongodb::{Database, IndexModel};
use mongodb::bson::{doc};
use mongodb::options::IndexOptions;
use crate::models::waiters::{NewWaiter, Waiter, WaiterAPI, WaiterId};
use crate::repo::repository::Repository;
use crate::services::error::ServiceError;

#[get("/waiters")]
pub(crate) async fn get_all_waiters(repo: web::Data<Repository>) -> Result<HttpResponse, ServiceError> {
    let result = repo.query_all::<WaiterAPI>().await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/waiters")]
pub(crate) async fn add_waiter(repo: web::Data<Repository>, data: web::Json<NewWaiter>) -> Result<HttpResponse, ServiceError> {
    let new_waiter = Waiter {
        _id: WaiterId::new(),
        name: data.name.clone(),
        code: data.code.clone(),
    };

    repo.insert_one::<Waiter>(new_waiter.clone()).await?;

    Ok(HttpResponse::Ok().json(new_waiter))
}

#[get("/waiters/{code}")]
pub(crate) async fn get_waiter(repo: web::Data<Repository>, code: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let code = code.into_inner();

    let result = repo
        .get_collection::<Waiter>()
        .find_one(doc! { "code": &code }, None).await;
    match result {
        Ok(Some(waiter)) => Ok(HttpResponse::Ok().json(waiter)),
        Ok(None) => Err(ServiceError::NotFound(format!("Waiter with code {} not found", code))),
        Err(err) => Err(ServiceError::InternalError(err.to_string())),
    }
}

pub(crate) async fn create_waiter_code_index(database: &Database) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "code": 1 })
        .options(options)
        .build();
    database
        .collection::<Waiter>("waiters")
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}
