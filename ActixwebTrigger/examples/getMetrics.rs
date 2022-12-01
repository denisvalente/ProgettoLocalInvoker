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
use actix_web_prom::PrometheusMetrics;
use std::{sync::Arc, thread};

#[get("/test")]
async fn test() -> impl Responder {

    format!("Hello test ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(test)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
