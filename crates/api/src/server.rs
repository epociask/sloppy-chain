
use std::sync::Arc;
use state::NodeState;
use config::Config;

use actix_web::{web, web::Data, middleware::Logger, App, HttpServer};

use crate::handlers;


#[actix_web::main]
pub async fn server(api_host: &str, api_port: u16, state: Arc<NodeState>) -> std::io::Result<()> {
    // TODO - incorporate the use of a router module
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .route("/transfer_tx", web::post().to(handlers::tx::submit_tx))
            .route("/mempool", web::get().to(handlers::tx::get_mempool_size))
            .route("/health", web::get().to(handlers::health::get_health))
            .wrap(Logger::default())
            })
        .workers(10)
        .bind((api_host, api_port))?
        .run()
        .await
    }

