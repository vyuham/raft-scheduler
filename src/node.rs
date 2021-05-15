use crate::raft_proto::{
    raft_client::RaftClient,
    raft_server::{Raft, RaftServer},
    Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
};
use rand::Rng;
use std::{cmp::min, collections::HashMap, error::Error, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{Duration, Instant},
};
use tonic::{transport::Server, Request, Response, Status};

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
    details: Arc<RaftDetails<T>>,
}

struct RaftDetails<T> {
    pub current_term: u64,
    pub commit_index: Mutex<u64>,
    pub voted_for: u8,
    pub votes_recieved: HashMap<u8, bool>,
    pub state: ServerState,
    pub id: u8,
    pub log: Mutex<Vec<(u64, T)>>,
    pub cluster: Vec<String>,
}

impl<T: RaftData + Sync + Send + 'static> RaftDetails<T> {
    pub fn new(id: u8, cluster: Vec<String>) -> Self {
        Self {
            current_term: 0,
            commit_index: Mutex::new(0),
            voted_for: id,
            votes_recieved: HashMap::new(),
            state: ServerState::Follower,
            id,
            log: Mutex::new(vec![]),
            cluster,
        }
    }

    pub async fn run(&mut self, start: u64, end: u64) -> Result<(), Box<dyn Error>> {
        let (mut clock, mut rng) = (Instant::now(), rand::thread_rng());
        loop {
            if clock.elapsed() > Duration::from_secs(rng.gen_range(start..end)) {
                clock = Instant::now();
                self.start_election().await?;
            }
        }
    }

    async fn start_election(&mut self) -> Result<(), Box<dyn Error>> {
        self.voted_for = self.id;
        self.votes_recieved.insert(self.id, true);

        for node in self.cluster.iter_mut() {
            let res = RaftClient::connect(format!("http://{  }", node))
                .await?
                .request_vote(Request::new(VoteRequest {
                    term: self.current_term + 1,
                    id: self.id as u64,
                    last_index: self.log.lock().await.len() as u64,
                    last_term: self.current_term,
                }))
                .await;
        }

        Ok(())
    }
}

impl<T: RaftData + Send + Sync + 'static> RaftNode<T> {
    /// Starts a raft node, consisting of server and client gRPC stubs.
    pub async fn start(
        id: u8,
        local_addr: String,
        mut nodes: Vec<String>,
    ) -> Result<Self, Box<dyn Error>> {
        // Keep addr of all nodes but the current one in directory.
        nodes.retain(|x| *x != local_addr);

        // Create shared state
        let raft_details = Arc::new(RaftDetails::new(id, nodes));

        // State that is handed over the the server stub on this node
        let raft = Self {
            details: raft_details.clone(),
        };

        // Server runs on a background thread and handles calls to the node
        tokio::spawn(async move {
            Server::builder()
                .add_service(RaftServer::new(raft))
                .serve(local_addr.parse().unwrap())
                .await
                .unwrap();
        });

        Ok(Self {
            details: raft_details,
        })
    }
}

#[tonic::async_trait]
impl<T: Sync + Send + 'static> Raft for RaftNode<T> {
    async fn request_vote(
        &self,
        request: Request<VoteRequest>,
    ) -> Result<Response<VoteReply>, Status> {
        let request = request.into_inner();
        if request.term < self.details.current_term {
            return Ok(Response::new(VoteReply {
                term: self.details.current_term,
                grant: false,
            }));
        } else if self.details.voted_for == self.details.id {
            return Ok(Response::new(VoteReply {
                term: self.details.current_term,
                grant: true,
            }));
        }

        Ok(Response::new(VoteReply {
            term: self.details.current_term,
            grant: false,
        }))
    }

    async fn append_entries(
        &self,
        request: Request<EntryRequest>,
    ) -> Result<Response<EntryReply>, Status> {
        let request = request.into_inner();
        if request.term < self.details.current_term {
            return Ok(Response::new(EntryReply {
                term: self.details.current_term,
                success: false,
            }));
        } else if request.prev_index > *self.details.commit_index.lock().await {
            return Ok(Response::new(EntryReply {
                term: self.details.current_term,
                success: false,
            }));
        } else if request.commit_index > *self.details.commit_index.lock().await {
            let last_entry_index = match self.details.log.lock().await.last() {
                Some(entry) => entry.0,
                None => 0,
            };
            let mut commit_index = self.details.commit_index.lock().await;
            *commit_index = min(request.commit_index, last_entry_index);
        }

        Ok(Response::new(EntryReply {
            term: self.details.current_term,
            success: true,
        }))
    }

    async fn join(&self, request: Request<Byte>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null {}))
    }
}
