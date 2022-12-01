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

#[get("/test")]
async fn test(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let q = serde_json::to_string(&query.to_owned()).unwrap();
    format!("Hello test {:?}", q)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(test)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
