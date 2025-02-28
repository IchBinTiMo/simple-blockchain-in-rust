use actix_web::{web, App, HttpServer};
use block::Block;
use blockchain::Blockchain;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::{env, sync::{mpsc, Arc, Mutex}};
use api::{get_chain, add, hello};

mod block;
mod blockchain;
mod api;


// type SharedBlockChain = Arc<Mutex<Blockchain>>;

static BC_SERVER: Lazy<(mpsc::Sender<Block>, Mutex<mpsc::Receiver<Block>>)> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel();
    (tx, Mutex::new(rx))
});


#[actix_web::main]
async fn main() ->std::io::Result<()> {
    let blockchain: Arc<Mutex<Blockchain>> = Arc::new(Mutex::new(Blockchain::new()));

    dotenv().ok();

    let ip: String = env::var("IP").unwrap_or("127.0.0.1".to_string());
    let port: String = env::var("PORT").unwrap_or("8080".to_string());

    let addr: String = format!("{}:{}", ip, port);

    println!("{addr}");

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
