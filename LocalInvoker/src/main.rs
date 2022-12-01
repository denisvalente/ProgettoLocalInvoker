#[macro_use]
extern crate serde_derive;
extern crate rmpv;
extern crate rmp_serde as rmps;

use rmps::Serializer;
use serde::Serialize;

use std::{env, str};
use std::fs;
use std::fs::File;

use std::io::{BufRead, BufReader, Write};

use std::process::{Command, Stdio};
use std::time::{SystemTime};
use log::{debug, info};
use nats::{Connection};
//use crate::serde_json::Deserializer;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
enum Op {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
struct Request {
    op: Op,
    nomefile: String
}

async fn work(command: String) -> String {
    // Invoking the command
    let mut child = Command::new("/bin/bash").arg("-c").arg(&command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().unwrap();

    debug!("Child launched PID = {}", child.id());

    let child_in = child.stdin.as_mut().unwrap();
    let mut child_out = BufReader::new(child.stdout.unwrap()).lines();

    //  Receive
    let mut out = child_out.next().unwrap().unwrap();
    debug!("Request {:?}", out);
    out.remove(0);
    out.remove(out.len()-1);
    debug!("Request cleaned {:?}", out);

    let  req_serialized:Vec<u8> = out.split(",").map(|x| x.trim().parse().unwrap()).collect();
    debug!("Serialized request {:?}", req_serialized);

    let req: Request = rmp_serde::from_slice(&req_serialized).unwrap();
    debug!("Deserialized request {:?}", req);
    //let query_result;
    let mut result_serialized  = Vec::new();
    let path = format!("/home/fdlv/localstorage/{}",req.nomefile);
    let answer = match req.op{
        Op::Read=>{
            // Send back a Vec<Row> to keep the invoker independent from the data type
            let start_time = SystemTime::now();

            println!("In file {}", req.nomefile);
            let data = fs::read_to_string(path).expect("Should have been able to read the file");
            println!("With text:\n{data}");

            let db_latency = SystemTime::now().duration_since(start_time).unwrap();
            info!("[DB_LATENCY] latency {} μs", db_latency.as_micros());

            let mut query_string :Vec<String>= Vec::new();
            query_string.push(format!("{:?}", data));
            debug!("Result: {:?}", query_string);
            query_string.serialize(&mut Serializer::new(&mut result_serialized)).unwrap();
            debug!("Result serialized: {:?}", result_serialized);

            // Conversion to string
            let mut string_result_serialized = String::new();
            let mut first = true;

            for el in result_serialized {
                if first {
                    string_result_serialized= format!("{}", el);
                    first = false;
                }
                else {
                    string_result_serialized = format!("{}, {}", string_result_serialized, el);
                }
            }
            string_result_serialized
        },
        Op::Create | Op::Update => {
            let start_time = SystemTime::now();
          
            let mut file = File::create(path).expect("Error encountered while creating file!");
            let content= "a".repeat(1024);
            file.write_all(content.as_bytes()).expect("Error while writing to file");
            let db_latency = SystemTime::now().duration_since(start_time).unwrap();
            info!("[DB_LATENCY] latency {} μs", db_latency.as_micros());

            "success".to_string()
        },
        Op::Delete => {
            let start_time = SystemTime::now();
            if std::path::Path::new(&path).exists() {
                fs::remove_file(path).expect("File delete failed");
                println!("File deleted successfully!");
            }else{
                println!("File not found!");
            }
            
            let db_latency = SystemTime::now().duration_since(start_time).unwrap();
            info!("[DB_LATENCY] latency {} μs", db_latency.as_micros());

            "success".to_string()
        },
    };
    //  Send back an answer
    debug!("Sent the result {}", answer);

    let _n= child_in.write_all(answer.as_str().as_bytes());
    let _r= child_in.write("\n".as_bytes());

    //  Return the child's output
    let res = child_out.next().unwrap().unwrap();
    debug!("Child output: {}", res);
    return res;
}

async fn server (nc: Connection, trigger_command: String, group: String, op: String){
    let mut n_reqs = 0;
    let mut total_latency = 0;
    let mut max = 0;
    let mut min = 0;

    let sub_command = nc.queue_subscribe(trigger_command.as_str(), group.as_str()).unwrap();
    debug!("Sub to command topic {:?}", sub_command);

    let command = "../GenericFunctionWithFlag/target/debug/genericfunctionwithflag";
    let args = format!("--operation {} --nomefile prova",op);

    loop {
        // Consuming message
        let mex = sub_command.next().unwrap();
        let received_id = mex.data;
        let reply =mex.reply.unwrap();
        let req_id = String::from_utf8(received_id).expect("Found invalid UTF-8");
        //let id_args = format!("{} --id {}",args, req_id);
        let id_args = format!("{}{} --id {}",args, req_id, req_id); //creo file che si chiama nome+id
        let mex = format!("{} {}", command, id_args);
        debug!("New request: {} env command: {}", req_id, mex);

        // Launch operation
        n_reqs = n_reqs +1;
        let start_time = SystemTime::now();
        let child_out = work(mex).await;
        let work_latency = SystemTime::now().duration_since(start_time).unwrap();
        debug!("Child output: {}",child_out);

        // Answer to stresser
        nc.publish(&reply, child_out).unwrap();
        debug!("Answer to {} sent", req_id);

        // Update general stats work
        total_latency = total_latency + work_latency.as_micros();
        if work_latency.as_micros() > max{
            max = work_latency.as_micros();
        }
        if work_latency.as_micros() < min || min == 0{
            min = work_latency.as_micros();
        }
        let average = total_latency/(n_reqs as u128);

        // Print Stats
        info!("[WORK_LATENCY] request number {}: latency {} μs", n_reqs, work_latency.as_micros());
        info!("[WORK_AVERAGE_LATENCY] request number {}: average latency {} μs", n_reqs, average);
        info!("[WORK_MIN_LATENCY] request number {}: {} μs", n_reqs, min);
        info!("[WORK_MAX_LATENCY] request number {}: max latency {} μs", n_reqs, max);
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let nats_server = env::var("NATSSERVER").unwrap_or("192.168.150.81".to_string());
    let trigger_command = env::var("TRIGGER").unwrap_or("topic2".to_string());
    let group = env::var("GROUP").unwrap_or("default".to_string());
    let op =env::var("OP").unwrap_or("Delete".to_string());

    let nc = nats::connect(nats_server.as_str()).unwrap();
    debug!("Connected to NATS {:?} ", nc);

    server(nc, trigger_command, group, op).await;
}
