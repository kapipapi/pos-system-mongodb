use actix_web::{get, HttpResponse, post, web};
use crate::models::tables::{NewTable, Table, TableId};
use crate::repo::repository::Repository;
use crate::services::error::ServiceError;

#[get("/tables")]
pub(crate) async fn get_all_tables(repo: web::Data<Repository>) -> Result<HttpResponse, ServiceError> {
    let result = repo.query_all::<Table>().await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/tables")]
pub(crate) async fn add_table(repo: web::Data<Repository>, data: web::Json<NewTable>) -> Result<HttpResponse, ServiceError> {
    let new_table = Table {
        _id: TableId::new(),
        name: data.name.clone(),
        x: data.x,
        y: data.y,
        level: data.level,
    };

    repo.insert_one::<Table>(new_table.clone()).await?;

    Ok(HttpResponse::Ok().json(new_table))
}

#[get("/tables/{id}")]
pub(crate) async fn get_table(repo: web::Data<Repository>, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let id = TableId::parse_str(&id.into_inner()).unwrap();

    let result = repo.query_one::<Table>(&id).await?;

    Ok(HttpResponse::Ok().json(result))
}
