use async_raft::{
    async_trait,
    raft::{Entry, MembershipConfig},
    storage::{CurrentSnapshotData, HardState, InitialState},
    RaftStorage,
};

use std::io::{Cursor, Error};

use crate::node::{RaftRequest, RaftResponse};

pub struct RaftLog;

#[async_trait::async_trait]
impl RaftStorage<RaftRequest, RaftResponse> for RaftLog {
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
