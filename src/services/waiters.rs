use actix_web::{get, post, web, HttpResponse};
use mongodb::{Database, IndexModel};
use mongodb::bson::{doc};
use mongodb::options::IndexOptions;
use crate::models::waiters::{NewWaiter, Waiter, WaiterId};
use crate::repo::repository::Repository;

#[get("/waiters")]
pub(crate) async fn get_all_waiters(repo: web::Data<Repository>) -> HttpResponse {
    let result = repo.query_all::<Waiter>().await;
    match result {
        Ok(waiters) => HttpResponse::Ok().json(waiters),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/waiters")]
pub(crate) async fn add_waiter(repo: web::Data<Repository>, data: web::Json<NewWaiter>) -> HttpResponse {
    let new_waiter = Waiter {
        _id: WaiterId::new(),
        name: data.name.clone(),
        code: data.code.clone(),
    };

    let result = repo.insert_one::<Waiter>(new_waiter.clone()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(new_waiter),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/waiters/{code}")]
pub(crate) async fn get_waiter(repo: web::Data<Repository>, code: web::Path<String>) -> HttpResponse {
    let code = code.into_inner();

    let result = repo
        .get_collection::<Waiter>()
        .find_one(doc! { "code": &code }, None).await;
    match result {
        Ok(Some(waiter)) => HttpResponse::Ok().json(waiter),
        Ok(None) => HttpResponse::NotFound().body(format!("No waiter found with code: {code}")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
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
