use actix_web::middleware::Logger;
use actix_web::{
    get, post,
    web::{self, Data, Path},
    App, HttpResponse, HttpServer, Responder,
};
use dashmap::DashMap;
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    time::Duration,
};
use std::{sync::Arc, thread};

#[post("/test")]
async fn test(query: web::Json<HashMap<String, String>>) -> impl Responder {
    let q = serde_json::to_string(&query.to_owned()).unwrap();
    format!("Hello test {:?}", q)
}

#[post("/test2")]
async fn test2(query: String) -> impl Responder {
    let q = serde_json::to_string(&query.to_owned()).unwrap();
    println!("{:?}", query);
    format!("Hello test {:?}", query)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(test)
            .service(test2)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
