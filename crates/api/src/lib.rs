use state::NodeState;
use actix_web::{post, web, App, HttpServer, HttpResponse, Result, Responder};
use actix_web::web::Data;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use primitives::tx::TransferTx;

async fn transfer_tx_post(json_tx: web::Json<TransferTx>,  node_state: Data<Arc<NodeState>>) -> HttpResponse {
    // 1. Deserialize transaction

    if let transaction = json_tx {
        // 2. Validate Tx
        // 3. Attempt to send Tx to mempool

        let mut mempool = node_state.mem_pool.lock().unwrap();
        let result = mempool.insert_tx(transaction.into_inner());

        return match result {
            Ok(()) => HttpResponse::Ok().body("data"),
            Err(msg) => HttpResponse::Ok().body("data"),
        }
    }
    
    
    return HttpResponse::Ok().body("data");
}

async fn get_mempool_size(node_state: Data<Arc<NodeState>>) -> HttpResponse {
    let mut mempool = node_state.mem_pool.lock().unwrap();
    
    let size = mempool.size().to_string();

    return HttpResponse::Ok().body(size);
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {

    let state = Data::new(Arc::new(NodeState::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/transfer_tx", web::post().to(transfer_tx_post))
            .route("/mempool", web::get().to(get_mempool_size))
            })
        .workers(10)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }