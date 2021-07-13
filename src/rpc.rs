use async_raft::{
    async_trait,
    raft::{
        AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest,
        InstallSnapshotResponse, VoteRequest, VoteResponse,
    },
    NodeId, RaftNetwork,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request};

use crate::node::RaftRequest;

mod raft_proto {
    tonic::include_proto!("raft");
}

use raft_proto::{raft_client::RaftClient, Byte};

mod client;
mod server;

use client::RaftClientStub;
use server::RaftServerStub;
pub struct RaftRPC {
    pub client: RaftClientStub,
}

impl RaftRPC {
    pub async fn new(self_addr: String, client_addrs: Vec<String>) -> Self {
        let mut clients = HashMap::new();
        for addr in client_addrs {
            clients.insert(addr.clone(), try_add(addr).await);
        }

        for (_, c) in clients.iter_mut() {
            match c
                .join(Request::new(Byte {
                    body: self_addr.clone().into_bytes(),
                }))
                .await
            {
                Ok(_) => println!("r"),
                Err(_) => println!("Error joining"),
            }
        }

        let clients = Arc::new(Mutex::new(clients));
        let clients_clone = clients.clone();

        tokio::spawn(async move {
            RaftServerStub::new(clients_clone)
                .start_server(self_addr)
                .await;
        });

        Self {
            client: RaftClientStub::new(clients).await,
        }
    }
}

pub async fn try_add(addr: String) -> RaftClient<Channel> {
    loop {
        match RaftClient::connect(format!("http://{}", addr)).await {
            Ok(c) => {
                println!("connected to {}", addr);
                return c;
            }
            Err(_) => continue,
        }
    }
}

#[async_trait::async_trait]
impl RaftNetwork<RaftRequest> for RaftRPC {
    async fn append_entries(
        &self,
        target: NodeId,
        rpc: AppendEntriesRequest<RaftRequest>,
    ) -> anyhow::Result<AppendEntriesResponse> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn install_snapshot(
        &self,
        target: NodeId,
        rpc: InstallSnapshotRequest,
    ) -> anyhow::Result<InstallSnapshotResponse> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn vote(&self, target: NodeId, rpc: VoteRequest) -> anyhow::Result<VoteResponse> {
        Err(anyhow::anyhow!("Nah"))
    }
}
