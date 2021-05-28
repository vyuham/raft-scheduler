use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub election_timeout_min: u64,
    pub election_timeout_max: u64,
    pub heartbeat_interval: u64,
}

impl Config {
    pub fn new_rand_election_timeout(&self) -> u64 {
        rand::thread_rng().gen_range(self.election_timeout_min..self.election_timeout_max)
    }

    pub fn new(
        election_timeout_min: u64,
        election_timeout_max: u64,
        heartbeat_interval: u64,
    ) -> Self {
        Self {
            election_timeout_min,
            election_timeout_max,
            heartbeat_interval,
        }
    }
}
