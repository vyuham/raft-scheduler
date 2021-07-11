use async_raft::{
    async_trait,
    raft::{
        AppendEntriesRequest, AppendEntriesResponse, Entry, InstallSnapshotRequest,
        InstallSnapshotResponse, MembershipConfig, VoteRequest, VoteResponse,
    },
    storage::{CurrentSnapshotData, HardState, InitialState},
    AppData, AppDataResponse, NodeId, RaftNetwork, RaftStorage,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Cursor, Error},
    sync::Arc,
};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request};

use crate::raft_proto::{raft_client::RaftClient, Byte};

pub mod client;
pub mod server;

use client::RaftClientStub;
use server::RaftServerStub;

pub struct RaftNode {
    pub client: RaftClientStub,
}

impl RaftNode {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftRequest {
    data: Vec<u8>,
}

impl AppData for RaftRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaftResponse(Option<String>);

impl AppDataResponse for RaftResponse {}

#[async_trait::async_trait]
impl RaftNetwork<RaftRequest> for RaftNode {
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

#[async_trait::async_trait]
impl RaftStorage<RaftRequest, RaftResponse> for RaftNode {
    type ShutdownError = Error;
    type Snapshot = Cursor<Vec<u8>>;

    async fn append_entry_to_log(&self, entry: &Entry<RaftRequest>) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn apply_entry_to_state_machine(
        &self,
        index: &u64,
        data: &RaftRequest,
    ) -> anyhow::Result<RaftResponse> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn create_snapshot(&self) -> anyhow::Result<(String, Box<Self::Snapshot>)> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn delete_logs_from(&self, start: u64, stop: Option<u64>) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn do_log_compaction(&self) -> anyhow::Result<CurrentSnapshotData<Self::Snapshot>> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn finalize_snapshot_installation(
        &self,
        index: u64,
        term: u64,
        delete_through: Option<u64>,
        id: String,
        snapshot: Box<Self::Snapshot>,
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn get_current_snapshot(
        &self,
    ) -> anyhow::Result<Option<CurrentSnapshotData<Self::Snapshot>>> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn get_initial_state(&self) -> anyhow::Result<InitialState> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn get_log_entries(
        &self,
        start: u64,
        stop: u64,
    ) -> anyhow::Result<Vec<Entry<RaftRequest>>> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn get_membership_config(&self) -> anyhow::Result<MembershipConfig> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn replicate_to_log(&self, entries: &[Entry<RaftRequest>]) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn replicate_to_state_machine(
        &self,
        entries: &[(&u64, &RaftRequest)],
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Nah"))
    }

    async fn save_hard_state(&self, hs: &HardState) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Nah"))
    }
}
