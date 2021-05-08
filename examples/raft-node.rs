use std::sync::Arc;
use raft::{RaftNode, RaftData};
use tokio::sync::Mutex;

struct Exec {}

impl RaftData for Exec {
    fn as_bytes(&self) -> Vec<u8> {
        vec![]
    }
}

#[tokio::main]
async fn main() {
    let log = Arc::new(Mutex::new(Vec::<(u64, Exec)>::new()));
    let cluster = vec![];
    let raft_node = RaftNode::new(0, log, cluster);
}
