use sha2::{Sha256, Digest};
use chrono::prelude::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};


// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u32,
    hash: String,
    previous_hash: String,
    data: i32,
    timestamp: String
}

struct Blockchain {
    blocks_map: HashMap<u32, Block>,
    blocks_vec: Vec<Block>
}

impl Block {
    fn new_block(index: u32, hash: String, previous_hash: String, data: i32, timestamp: String) -> Block {
        Block {
            index,
            hash,
            previous_hash,
            data,
            timestamp
        }
    }

    fn calculate_hash(&self) -> String {
        let record: String = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash);

        let mut h: Sha256 = Sha256::new();

        h.update(record.as_bytes());

        format!("{:x}", h.finalize())
    }

    fn new(data: i32, prev: &Block) -> Block {
        let index: u32 = prev.index + 1;
        let previous_hash: String = prev.hash.clone();
        let timestamp: String = Utc::now().timestamp().to_string();

        let record: String = format!("{}{}{}{}", index, timestamp, data, previous_hash);

        let mut h: Sha256 = Sha256::new();

        h.update(record.as_bytes());

        let hash: String = format!("{:x}", h.finalize());

        Block::new_block(index, hash, previous_hash, data, timestamp)
    }
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blocks_map: HashMap<u32, Block> = HashMap::new();
        let mut blocks_vec: Vec<Block> = Vec::new();

        let now: i64 = Utc::now().timestamp();

        let genesis: Block = Block::new_block(0, "".to_string(), "".to_string(), 0, now.to_string());

        blocks_map.insert(genesis.index, genesis.clone());
        blocks_vec.push(genesis);

        Blockchain {
            blocks_map,
            blocks_vec
        }
    }

    fn is_block_valid(&self, current: &Block, prev: &Block) -> bool {
        current.index == prev.index + 1 && 
        current.previous_hash == prev.hash && 
        current.hash == current.calculate_hash()
    }

    fn add_block(&mut self, data: i32) -> Result<&str, Error> {
        let prev: &Block = self.blocks_map.get(&(self.len() as u32 - 1)).unwrap();

        let new_block: Block = Block::new(data, prev);

        if !self.is_block_valid(&new_block, prev) {
            println!("Block not valid");
            Err(Error::new(ErrorKind::Other, "Block not valid"))
        } else {
            self.blocks_map.insert(new_block.index, new_block.clone());
            self.blocks_vec.push(new_block);
            println!("Block added");
            Ok("Block added")
        }
    }

    fn len(&self) -> usize {
        self.blocks_map.len()
    }

    fn get_chain(&self) -> Vec<Block> {
        self.blocks_vec.clone()
    }
}

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
