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

    pub async fn request(&mut self) {
        let mut clients = self.clients.lock().await;
        for (_, c) in clients.iter_mut() {
            match c
                .request_vote(Request::new(VoteRequest {
                    term: 1,
                    id: 1,
                    last_index: 0,
                    last_term: 0,
                }))
                .await
            {
                Ok(r) => println!("{:?}", r.into_inner()),
                Err(_) => println!("Error requesting"),
            }
        }
    }

    pub async fn append(&mut self) {
        let mut clients = self.clients.lock().await;
        for (_, c) in clients.iter_mut() {
            match c
                .append_entries(Request::new(EntryRequest {
                    term: 1,
                    id: 1,
                    prev_index: 0,
                    prev_term: 0,
                    entry: vec![0, 1, 2, 3],
                    commit_index: 0,
                }))
                .await
            {
                Ok(r) => println!("{:?}", r.into_inner()),
                Err(_) => println!("Error appending"),
            }
        }
    }
}
