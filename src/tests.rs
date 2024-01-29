use actix_web::{
    test::{call_and_read_body, call_and_read_body_json, init_service, TestRequest},
    web::Bytes,
};
use mongodb::Client;
use mongodb::options::{ClientOptions, Credential};
use crate::models::waiters::Waiter;

use super::*;

#[actix_web::test]
#[ignore = "requires MongoDB instance running"]
async fn test() {
    let uri = dotenvy::var("URI").expect("URI must be set");
    let username = dotenvy::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let password = dotenvy::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = dotenvy::var("DB_NAME").expect("DB_NAME must be set");

    let mut client_options = ClientOptions::parse_async(uri).await.unwrap();
    let default_cred = Credential::builder()
        .username(username)
        .password(password)
        .source(db_name)
        .build();
    client_options.credential = Some(default_cred);
    let client = Client::with_options(client_options).unwrap();

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(add_waiter)
            .service(get_waiter),
    )
        .await;

    let waiter = Waiter {
        name: "Kacper".into(),
        code: "1111".into(),
    };

    let req = TestRequest::post()
        .uri("/add_waiter")
        .set_form(&waiter)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"waiter added"));

    let req = TestRequest::get()
        .uri(&format!("/get_waiter/{}", &waiter.code))
        .to_request();

    let response: Waiter = call_and_read_body_json(&app, req).await;
    assert_eq!(response, waiter);
}