use envconfig::Envconfig;

#[derive(Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "API_HOST")]
    pub api_host: String,
    #[envconfig(from = "API_PORT")]
    pub api_port: u16,
    #[envconfig(from = "MAX_MEMPOOL")]
    pub max_mempool: usize,
    #[envconfig(from = "ECDSA_PUB_KEY")]
    pub pub_key: String,
    #[envconfig(from = "ECDSA_PRIV_KEY")]
    pub priv_key: String,
}