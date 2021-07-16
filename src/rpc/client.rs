use async_raft::raft::{AppendEntriesRequest, AppendEntriesResponse, EntryPayload};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::node::RaftRequest;

use super::raft_proto::{raft_client::RaftClient, EntryReply, EntryRequest};

pub struct RaftClientStub {
    clients: Arc<Mutex<HashMap<u64, RaftClient<Channel>>>>,
}

impl RaftClientStub {
    pub async fn new(clients: Arc<Mutex<HashMap<u64, RaftClient<Channel>>>>) -> Self {
        Self { clients }
    }

    pub async fn append_entries(
        &self,
        id: u64,
        request: AppendEntriesRequest<RaftRequest>,
    ) -> Option<AppendEntriesResponse> {
        let client = self.clients.lock().await;
        let AppendEntriesRequest {
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        } = request;

        let entries = entries
            .iter()
            .fold(vec![], |mut acc, e| match e.payload.clone() {
                EntryPayload::Normal(e) => {
                    acc.append(&mut e.data.to_bytes());
                    acc
                }
                _ => acc,
            });

        let request = EntryRequest {
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        };

        if let Some(c) = client.get(&id) {
            if let Ok(r) = c.clone().append_entries(request).await {
                let EntryReply { term, success } = r.into_inner();
                return Some(AppendEntriesResponse {
                    term,
                    success,
                    conflict_opt: None,
                });
            }
        }

        None
    }
}
