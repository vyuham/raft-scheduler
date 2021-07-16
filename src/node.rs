use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    sync::Arc,
};

use async_raft::{raft::Raft, AppData, AppDataResponse, Config};
use serde::{Deserialize, Serialize};

use crate::{log::RaftLog, rpc::RaftRPC};

pub struct RaftNode {
    pub raft: Raft<RaftRequest, RaftResponse, RaftRPC, RaftLog>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftRequest {
    data: Vec<u8>,
}

impl RaftRequest {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl AppData for RaftRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftResponse(Option<String>);

impl AppDataResponse for RaftResponse {}

impl RaftNode {
    pub async fn new(addr: String) -> Self {
        let id = Self::addr_id(addr);
        let config = Arc::new(Config::build("raex".to_string()).validate().unwrap());
        let rpc = Arc::new(RaftRPC::new("".to_string(), vec!["".to_string()]).await);
        let log = Arc::new(RaftLog);

        Self {
            raft: Raft::new(id, config, rpc, log),
        }
    }

    /// Converts addresses from String format into NodeId/u64 format
    /// NOTE: Due to the IPV6 standard format being 128 bit, 64 bits will be lost in conversion.
    pub fn addr_id(addr: String) -> u64 {
        let addr: SocketAddr = addr.parse().unwrap();
        (match addr.ip() {
            IpAddr::V4(ip) => ip.to_ipv6_mapped(),
            IpAddr::V6(ip) => ip,
        }
        .octets()
        .iter()
        .fold(0u64, |acc, &x| (acc << 8) + x as u64)
            << 16)
            + addr.port() as u64
    }

    /// Converts addresses from NodeId/u64 format back into String format
    pub fn id_addr(id: u64) -> String {
        let mut ip = id >> 16;
        let mut octets = [0u8; 16];
        for i in (0..16).rev() {
            octets[i] = ip as u8;
            ip >>= 8;
        }
        SocketAddr::new(IpAddr::V6(Ipv6Addr::from(octets)), id as u16).to_string()
    }
}
