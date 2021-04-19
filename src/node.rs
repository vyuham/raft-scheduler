use crate::{
    raft_proto::{raft_client::RaftClient, raft_server::RaftServer, EntryRequest},
    state::RaftState,
};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::{Channel, Server};

/// A trait to ensures interfaces necessart in types that can be transformed into byte based messages for
/// easy transport over the network, ensuring raft based consensus of cluster state.
pub trait RaftData {
    fn as_bytes(&self) -> Vec<u8>;
}

/// Details necessary to construct a node for raft consensus.
pub struct RaftNode<T> {
    raft: RaftState<T>,
    clients: Vec<RaftClient<Channel>>,
}

impl<T: RaftData + Sync + Send + 'static> RaftNode<T> {
    /// Starts a raft node, consisting of server and client gRPC stubs.
    pub async fn start(
        local_addr: String,
        mut nodes: Vec<String>,
        id: u8,
    ) -> Result<Self, Box<dyn Error>> {
        let log = Arc::new(Mutex::new(vec![]));

        // Keep addr of all nodes but the current one in directory.
        nodes.retain(|x| *x != local_addr);

        // Generate a list of client stubs to be used in communications later.
        let mut clients = vec![];
        for node in nodes {
            clients.push(RaftClient::connect(format!("http://{  }", node)).await?);
        }

        // State that is handed over the the server stub on this node
        let raft = RaftState::new(id, log.clone());

        // Server runs on a background thread and handles calls to the node
        tokio::spawn(async move {
            Server::builder()
                .add_service(RaftServer::new(raft))
                .serve(local_addr.parse().unwrap())
                .await
                .unwrap();
        });

        Ok(Self {
            raft: RaftState::new(id, log),
            clients,
        })
    }

    pub async fn schedule(&mut self, next: T) {
        let mut res = vec![];
        for client in self.clients.clone().iter_mut() {
            let commit_index = *self.raft.commit_index.lock().await;
            res.push(
                client
                    .append_entries(EntryRequest {
                        term: self.raft.current_term + 1,
                        id: self.raft.id as u64,
                        prev_index: commit_index,
                        prev_term: self.raft.current_term,
                        entry: next.as_bytes(),
                        commit_index: commit_index + 1,
                    })
                    .await,
            );
        }

        // TODO: Implement schedule()
    }

    pub async fn next(&self) {
        // TODO: Implement next()
    }
}
