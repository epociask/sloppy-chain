use std::sync::Arc;

use envconfig::Envconfig;
use env_logger::Env;

use dotenv;

use api;
use config;
use state;

fn main() {
    // Load env vars from configuration file
    dotenv::from_filename(".env").ok();
    
    // Configure env logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let cfg = config::Config::init().unwrap();

    let node_state = Arc::new(
        state::NodeState::new(cfg.max_mempool)
    );

    // TODO - Multithread api run with additional concurrent routines
    api::run(cfg.clone(), node_state);
    
}
