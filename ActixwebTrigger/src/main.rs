use actix_web::middleware::Logger;
use actix_web::{
    get, post,
    web::{self, Data, Path},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_prom::PrometheusMetrics;
use async_nats::Connection;
use dashmap::DashMap;
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    time::Duration,
};
use std::{sync::Arc, thread, time};

async fn query(
    function: &str,
    map: Data<DashMap<String, String>>,
    message: &str,
    nc: web::Data<Connection>,
) -> Result<async_nats::Message, std::io::Error> {
    match map.get(function) {
        Some(trig) => {
            Some(format!("{:?}", *trig));
            nc.request(&*trig, message).await
        }
        None => Err(Error::new(ErrorKind::Other, "oh no!")),
    }
}

async fn send(
    function: &str,
    map: Data<DashMap<String, String>>,
    message: &str,
    nc: web::Data<Connection>,
) -> Result<(), std::io::Error> {
    match map.get(function) {
        Some(trig) => {
            Some(format!("{:?}", *trig));
            nc.publish(&*trig, message).await
        }
        None => Err(Error::new(ErrorKind::Other, "oh no!")),
    }
}

#[get("/test/")]
async fn test() -> impl Responder {
    format!("Hello test")
}

#[get("/function/{function}/")]
async fn functiong(
    Path(function): Path<String>,
    map: web::Data<DashMap<String, String>>,
    httpquery: web::Query<HashMap<String, String>>,
    nc: web::Data<Connection>,
) -> impl Responder {
    let q = serde_json::to_string(&httpquery.to_owned()).unwrap();
    let resp = query(&function, map, &q, nc).await;
    match resp {
        Ok(message) => {
            let msg = String::from_utf8_lossy(&message.data).to_string();
            HttpResponse::Ok().body(msg)
        }
        Err(_) => HttpResponse::Ok().body("Error"),
    }
}

#[post("/function/{function}/")]
async fn functionp(
    web::Path(function): web::Path<String>,
    map: web::Data<DashMap<String, String>>,
    httbody: String,
    nc: web::Data<Connection>,
) -> impl Responder {
    let message = httbody;
    let resp = query(&function, map, &message, nc).await;
    match resp {
        Ok(message) => {
            let msg = String::from_utf8_lossy(&message.data).to_string();
            HttpResponse::Ok().body(msg)
        }
        Err(_) => HttpResponse::Ok().body("Error"),
    }
}

#[get("/asyncfunction/{function}/")]
async fn asyncfunctiong(
    Path(function): Path<String>,
    map: web::Data<DashMap<String, String>>,
    httpquery: web::Query<HashMap<String, String>>,
    nc: web::Data<Connection>,
) -> impl Responder {
    let q = serde_json::to_string(&httpquery.to_owned()).unwrap();
    send(&function, map, &q, nc).await;
    HttpResponse::Ok().body("ok")
}

#[post("/asyncfunction/{function}/")]
async fn asyncfunctionp(
    web::Path(function): web::Path<String>,
    map: web::Data<DashMap<String, String>>,
    httbody: String,
    nc: web::Data<Connection>,
) -> impl Responder {
    let message = httbody;
    info!("{:?}", message);
    send(&function, map, &message, nc).await;

    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let map: web::Data<DashMap<String, String>> = web::Data::new(DashMap::new());
    //let mymap = map.clone();
    let tmap = map.clone();
    thread::spawn(move || puller(tmap));

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    let NATSSERVER = option_env!("NATSSERVER").or(Some("localhost")).unwrap();
    let TRIGGERPORT = option_env!("TRIGGERPORT").or(Some("8081")).unwrap();
    let addr = format!("0.0.0.0:{}",TRIGGERPORT);
    let nc = async_nats::connect(NATSSERVER).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(map.clone())
            .data(nc.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(prometheus.clone())
            .service(functiong)
            .service(functionp)
            .service(asyncfunctiong)
            .service(asyncfunctionp)
            .service(test)
    })
    .bind(addr)?
    .run()
    .await
}

//[["dummypipe",{"triggerEndpoyntType":["http"],"medium":"nats","topic":"dummy"}]]
fn puller(
    map: Data<DashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let RULESSERVERHOST = option_env!("RULESSERVERHOST")
        .or(Some("192.168.150.81"))
        .unwrap();
    let RULESSERVERPORT = option_env!("RULESSERVERPORT").or(Some("3000")).unwrap();
    let RULESSERVERPATH = option_env!("RULESSERVERPATH")
        .or(Some("trigger/rules"))
        .unwrap();
    let scheduling = option_env!("RULESFETCHINGINTERVAL")
        .or(Some("2000"))
        .unwrap();
    let RULESRESCMS = time::Duration::from_millis(scheduling.parse().unwrap());
    let ID = option_env!("ID").or(Some("basicHttpTrigger")).unwrap();

    let url = format!(
        "http://{}:{}/{}/{}",
        RULESSERVERHOST, RULESSERVERPORT, RULESSERVERPATH, ID
    );
    info!("call url:{}", url);
    let client = reqwest::blocking::Client::new();

    loop {
        let req = client.get(&url).timeout(Duration::new(5, 0)).send()?;

        let rules = req.json::<Vec<Rule>>()?;

        for rule in rules {
            if !map.contains_key(&rule.id) {
                info!("added rule");
                map.insert(rule.id, rule.rule.topic);
            }
        }
        thread::sleep(RULESRESCMS);
    }
    //Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Rule {
    id: String,
    rule: Trigger,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Trigger {
    triggerEndpoyntType: Vec<String>,
    medium: String,
    topic: String,
}
