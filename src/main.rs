use std::sync::Arc;
use std::thread;

use envconfig::Envconfig;
use env_logger::Env;

use dotenv;

use api;
use config;
use p2p;
use state;

fn main() {
    // Load env vars from configuration file
    dotenv::from_filename(".env").ok();
    
    // Configure env logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let cfg = config::Config::init_from_env().unwrap();
    let cloned_cfg = cfg.clone();

    let node_state = Arc::new(
        state::NodeState::new(cfg.max_mempool())
    );

    // TODO - Multithread api run with additional concurrent routines
    let handle = thread::spawn(|| {
        let result = api::run(cloned_cfg, node_state);

        match result {
            Ok(_) => println!("API successfully finished running"),
            Err(err) => println!("Got error {} ", err),
        }
    });


    let opt = p2p::main(&cfg.peer_ids());

    handle.join().unwrap();

}
