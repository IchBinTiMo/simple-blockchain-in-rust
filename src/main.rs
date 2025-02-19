use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};

mod block;
mod blockchain;

use crate::blockchain::Blockchain;

type SharedBlockChain = Arc<Mutex<Blockchain>>;

// Backend
#[get ("/")]
async fn get_chain(blockchain: web::Data<SharedBlockChain>) -> impl Responder {
    if let Ok(chain) = blockchain.lock() {
        HttpResponse::Ok().json(chain.get_chain())
    } else {
        println!("Failed to lock blockchain");
        HttpResponse::BadRequest().body("Failed to lock blockchain")
    }
}

#[get ("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post ("/add_block")]
async fn add(data: web::Json<i32>, blockchain: web::Data<SharedBlockChain>) -> impl Responder {
    if let Ok(mut chain) = blockchain.lock() {
        let data: i32 = data.into_inner();
        match chain.add_block(data) {
            Ok(msg) => HttpResponse::Ok().body(msg.to_string()),
            Err(msg) => HttpResponse::BadRequest().body(msg.to_string())
        }
    } else {
        println!("Failed to lock blockchain");
        HttpResponse::BadRequest().body("Failed to lock blockchain")
    }
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .service(get_chain)
            .service(add)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
