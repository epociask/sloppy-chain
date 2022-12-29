#![allow(unused_mut)]

use state::NodeState;
use primitives::tx::TransferTx;

use actix_web::{web, HttpResponse};
use actix_web::web::Data;
use std::sync::{Arc};


pub async fn submit_tx(json_tx: web::Json<TransferTx>,  node_state: Data<Arc<NodeState>>) -> HttpResponse {
    // 1. Deserialize transaction

    if let transaction = json_tx {
        // 2. Validate Tx
        
        // 3. Attempt to send Tx to mempool
        let mut mempool = node_state.mem_pool.lock().unwrap();

        let result = mempool.insert_tx(transaction.into_inner());

        return match result {
            Ok(()) => HttpResponse::Ok().body("Transaction accepted"),
            Err(msg) => HttpResponse::InternalServerError().
            body(format!("Could not process transaction to mempool: {}", msg)),
        }
    }
    
        
    HttpResponse::BadRequest().
    body("Could not process JSON transaction object supplied in request body.")
}

pub async fn get_mempool_size(node_state: Data<Arc<NodeState>>) -> HttpResponse {
    let mut mempool = node_state.mem_pool.lock().unwrap();
    
    let size = mempool.size().to_string();

    return HttpResponse::Ok().body(size);
}