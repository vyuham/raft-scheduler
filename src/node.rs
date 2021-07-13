use std::sync::Arc;

use async_raft::{raft::Raft, AppData, AppDataResponse, Config};
use serde::{Deserialize, Serialize};

use crate::{log::RaftLog, rpc::RaftRPC};

pub struct RaftNode {
    raft: Raft<RaftRequest, RaftResponse, RaftRPC, RaftLog>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftRequest {
    data: Vec<u8>,
}

impl AppData for RaftRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftResponse(Option<String>);

impl AppDataResponse for RaftResponse {}

impl RaftNode {
    pub async fn new() -> Self {
        let id = 0;
        let config = Arc::new(Config::build("raex".to_string()).validate().unwrap());
        let rpc = Arc::new(RaftRPC::new("".to_string(), vec!["".to_string()]).await);
        let log = Arc::new(RaftLog);

        Self {
            raft: Raft::new(id, config, rpc, log),
        }
    }
}
