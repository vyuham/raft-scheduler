use async_raft::{AppData, NodeId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{raft::RaftLog, state::RaftStateMachine};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftData {}

impl AppData for RaftData {}

/// Details necessary to construct a node for raft consensus.
pub struct RaftNode {
    state: Arc<Mutex<RaftStateMachine>>,
    log: Arc<Mutex<RaftLog>>,
}

pub fn ip_node(ip: &str) -> NodeId {
    let mut ip: Vec<&str> = ip.split(":").collect();
    let mut id = ip[1].parse::<u64>().unwrap();
    ip = ip[0].split(".").collect();
    id += {
        let (mut id, mut x) = (0, 56u64);
        for i in 0..4 {
            id += ip[i].parse::<u64>().unwrap() << x;
            x -= 8;
        }
        id
    };

    id
}

pub fn node_ip(id: NodeId) -> String {
    format!(
        "{}.{}.{}.{}:{}", // aaa.bbb.ccc.ddd:eee...
        id >> 56 as u8,   // aaa
        id >> 48 as u8,   // bbb
        id >> 40 as u8,   // ccc
        id >> 32 as u8,   // ddd
        id as u32,        // eee...
    )
}
