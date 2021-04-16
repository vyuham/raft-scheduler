use crate::{
    raft_proto::{raft_client::RaftClient, raft_server::RaftServer},
    state::RaftState,
};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    sync::Arc,
};
use tokio::sync::Mutex;
use tonic::transport::{Channel, Server};

pub struct RaftNode<T> {
    raft: RaftState<T>,
    clients: Vec<RaftClient<Channel>>,
}

impl<T: Sync + Send + 'static> RaftNode<T> {
    pub async fn start(local_addr: String, mut nodes: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let (schedule, executing, free_nodes, log) = (
            Arc::new(Mutex::new(VecDeque::new())),
            Arc::new(Mutex::new(HashMap::new())),
            Arc::new(Mutex::new(vec![])),
            Arc::new(Mutex::new(vec![])),
        );

        nodes.retain(|x| *x != local_addr);

        let mut clients = vec![];

        for node in nodes {
            clients.push(RaftClient::connect(format!("http://{  }", node)).await?);
        }

        // State that is handed over the the server stub on this node
        let raft = RaftState::new(
            schedule.clone(),
            executing.clone(),
            free_nodes.clone(),
            log.clone(),
        );

        // Server runs on a background thread and handles calls to the node
        tokio::spawn(async move {
            Server::builder()
                .add_service(RaftServer::new(raft))
                .serve(local_addr.parse().unwrap())
                .await
                .unwrap();
        });

        Ok(Self {
            raft: RaftState::new(schedule, executing, free_nodes, log),
            clients,
        })
    }

    pub async fn schedule(&self, next: T) {
        self.raft.schedule.lock().await.push_back(next);
    }

    pub async fn next(&self) {
        if let Some(id) = self.raft.free_nodes.lock().await.pop() {
            if let Some(next) = self.raft.schedule.lock().await.pop_front() {
                self.raft.executing.lock().await.insert(id, next);
            }
        }
    }
}
