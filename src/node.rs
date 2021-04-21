use crate::raft_proto::{
    raft_client::RaftClient,
    raft_server::{Raft, RaftServer},
    Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
};
use std::{cmp::min, error::Error, sync::Arc};
use tokio::sync::Mutex;
use tonic::{
    transport::{Channel, Server},
    Request, Response, Status,
};

/// A trait to ensures interfaces necessart in types that can be transformed into byte based messages for
/// easy transport over the network, ensuring raft based consensus of cluster state.
pub trait RaftData {
    fn as_bytes(&self) -> Vec<u8>;
}

/// Possible server states within a raft cluster
/// Follower: Can only respond to requests from nodes of cluster
/// Candidate: Can only request to be elected Leader of cluster
/// Leader: Operate until node failure, leads updates to state
pub enum ServerState {
    Follower,
    Candidate,
    Leader,
}

/// Details necessary to construct a node for raft consensus.
pub struct RaftNode<T> {
    current_term: u64,
    commit_index: Arc<Mutex<u64>>,
    voted_for: u8,
    state: ServerState,
    id: u8,
    log: Arc<Mutex<Vec<(u64, T)>>>,
    clients: Vec<RaftClient<Channel>>,
}

impl<T: RaftData + Sync + Send + 'static> RaftNode<T> {
    pub fn new(id: u8, log: Arc<Mutex<Vec<(u64, T)>>>, clients: Vec<RaftClient<Channel>>) -> Self {
        Self {
            current_term: 0,
            commit_index: Arc::new(Mutex::new(0)),
            voted_for: id,
            state: ServerState::Follower,
            id,
            log,
            clients,
        }
    }

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
        let raft = Self::new(id, log.clone(), clients.clone());

        // Server runs on a background thread and handles calls to the node
        tokio::spawn(async move {
            Server::builder()
                .add_service(RaftServer::new(raft))
                .serve(local_addr.parse().unwrap())
                .await
                .unwrap();
        });

        Ok(Self::new(id, log.clone(), clients))
    }

    pub async fn schedule(&mut self, next: T) {
        let mut res = vec![];
        for client in self.clients.clone().iter_mut() {
            let commit_index = *self.commit_index.lock().await;
            res.push(
                client
                    .append_entries(EntryRequest {
                        term: self.current_term + 1,
                        id: self.id as u64,
                        prev_index: commit_index,
                        prev_term: self.current_term,
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

#[tonic::async_trait]
impl<T: Sync + Send + 'static> Raft for RaftNode<T> {
    async fn request_vote(
        &self,
        request: Request<VoteRequest>,
    ) -> Result<Response<VoteReply>, Status> {
        let request = request.into_inner();
        if request.term < self.current_term {
            return Ok(Response::new(VoteReply {
                term: self.current_term,
                grant: false,
            }));
        } else if self.voted_for == self.id {
            return Ok(Response::new(VoteReply {
                term: self.current_term,
                grant: true,
            }));
        }

        Ok(Response::new(VoteReply {
            term: self.current_term,
            grant: false,
        }))
    }

    async fn append_entries(
        &self,
        request: Request<EntryRequest>,
    ) -> Result<Response<EntryReply>, Status> {
        let request = request.into_inner();
        if request.term < self.current_term {
            return Ok(Response::new(EntryReply {
                term: self.current_term,
                success: false,
            }));
        } else if request.prev_index > *self.commit_index.lock().await {
            return Ok(Response::new(EntryReply {
                term: self.current_term,
                success: false,
            }));
        } else if request.commit_index > *self.commit_index.lock().await {
            let last_entry_index = match self.log.lock().await.last() {
                Some(entry) => entry.0,
                None => 0,
            };
            let mut commit_index = self.commit_index.lock().await;
            *commit_index = min(request.commit_index, last_entry_index);
        }

        Ok(Response::new(EntryReply {
            term: self.current_term,
            success: true,
        }))
    }

    async fn join(&self, args: Request<Byte>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null {}))
    }
}
