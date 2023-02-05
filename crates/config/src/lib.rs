use envconfig::Envconfig;
use core::str::Split;

// Single config struct to store all env variables
#[derive(Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "API_HOST")]
    api_host: String,
    #[envconfig(from = "API_PORT")]
    api_port: u16,
    #[envconfig(from = "MAX_MEMPOOL")]
    max_mempool: usize,
    #[envconfig(from = "ECDSA_PUB_KEY")]
    pub_key: String,
    #[envconfig(from = "ECDSA_PRIV_KEY")]
    priv_key: String,
    #[envconfig(from = "PEER_IDS")]
    peers_str: String
}

impl Config {
    pub fn api_host(&self) -> String {
        self.api_host.clone()
    }

    pub fn api_port(&self) -> u16 {
        self.api_port.clone()
    }

    pub fn max_mempool(&self) -> usize {
        self.max_mempool
    }

    pub fn public_key(&self) -> String {
        self.pub_key.clone()
    }

    pub fn private_key(&self) -> String {
        self.priv_key.clone()
    }

    pub fn peer_ids(&self) -> Vec<String> {
        let ids: Split<&str> = self.peers_str.split(",");
        let mut static_vec: Vec<String> = vec![];

        ids.for_each(|val: &str| {
            static_vec.push(val.into());
        });

        static_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_ids() {
        let cfg = Config{
            api_host: String::from(""), api_port: 0, 
            max_mempool: 0, pub_key: String::from(""),
            priv_key: String::from(""),
            peers_str: String::from("a,b,c"),
        };

        assert_eq!(cfg.peer_ids(), vec!["a", "b", "c"]);
    }
}