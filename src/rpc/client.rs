use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request};

use super::raft_proto::{raft_client::RaftClient, EntryRequest, VoteRequest};

pub struct RaftClientStub {
    clients: Arc<Mutex<HashMap<String, RaftClient<Channel>>>>,
}

impl RaftClientStub {
    pub async fn new(clients: Arc<Mutex<HashMap<String, RaftClient<Channel>>>>) -> Self {
        Self { clients }
    }
}
