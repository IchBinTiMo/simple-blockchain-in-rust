use actix_web::{web, get, post, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
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