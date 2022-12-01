use isahc::{HttpClient, Request};
use std::time;
use tokio::sync::mpsc::{channel, Sender};

async fn request_task(x:u64,i:i32,_sender: Sender<()>) {
    let client = HttpClient::builder()
       .default_header("Content-Type", "application/json")  
       .build().unwrap();
    let request = Request::post("http://192.168.150.81:8081/function/local/")
        .header("Content-Type", "application/json")
        .body(format!("{}-{}",x,i)).unwrap();
    let reqtime = time::SystemTime::now()
       .duration_since(time::UNIX_EPOCH)
       .expect("Time went backwards");
    println!("[REQUEST] {}-{}:{:?}",x,i,reqtime); 
    let response = client.send_async(request).await.unwrap();
  let resptime = time::SystemTime::now()
       .duration_since(time::UNIX_EPOCH)
       .expect("Time went backwards");
    println!("[RESPONSE] {} {}-{}:{:?}",response.status(),x,i,resptime);
                                                                                
}
 
#[tokio::main]   //crea un tokio runtime con 4 worker threads
async fn main() {
    let (send, mut recv) = channel(1);
    let sleep:u64 = 1000000;
    let batchsize:i32 = 100;
    let minsleep:u64 = 1000;
    let mut x:u64 = sleep;
    while x >= minsleep {
         let mut interval = tokio::time::interval(time::Duration::from_micros(x));
         for i in 0..batchsize {
            tokio::spawn(request_task(x,i,send.clone()));
            interval.tick().await;
         }
         x = x / 2;
     }
     drop(send);
     let _ = recv.recv().await; //quando tutti i sender vanno fuori scope (quando una task finisce) ritorna con un errore e sblocca l'await    
}
