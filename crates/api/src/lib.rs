use config::Config;
use state::NodeState;
use std::sync::Arc;

mod errors;
mod helpers;
pub mod handlers;
mod server;

use crate::server::server;

#[actix_rt::main]
pub async fn run(cfg: Config, node_state: Arc<NodeState>) -> std::io::Result<()> {
    server(&cfg.api_host, cfg.api_port, node_state)
}