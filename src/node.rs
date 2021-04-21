use crate::raft_proto::{
    raft_client::RaftClient,
    raft_server::{Raft, RaftServer},
    Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
};
use rand::Rng;
use std::{cmp::min, collections::HashMap, error::Error, sync::Arc};
use tokio::{sync::Mutex, time::{Instant, Duration}};
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
    votes_recieved: HashMap<u8, bool>,
    state: ServerState,
    id: u8,
    log: Arc<Mutex<Vec<(u64, T)>>>,
    cluster: Vec<RaftClient<Channel>>,
}

impl<T: RaftData + Sync + Send + 'static> RaftNode<T> {
    pub fn new(id: u8, log: Arc<Mutex<Vec<(u64, T)>>>, cluster: Vec<RaftClient<Channel>>) -> Self {
        Self {
            current_term: 0,
            commit_index: Arc::new(Mutex::new(0)),
            voted_for: id,
            votes_recieved: HashMap::new(),
            state: ServerState::Follower,
            id,
            log,
            cluster,
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
        let mut cluster = vec![];
        for node in nodes {
            cluster.push(RaftClient::connect(format!("http://{  }", node)).await?);
        }

        // State that is handed over the the server stub on this node
        let raft = Self::new(id, log.clone(), cluster.clone());

        // Server runs on a background thread and handles calls to the node
        tokio::spawn(async move {
            Server::builder()
                .add_service(RaftServer::new(raft))
                .serve(local_addr.parse().unwrap())
                .await
                .unwrap();
        });

        Ok(Self::new(id, log, cluster))
    }

    pub async fn run(&mut self, start: u64, end: u64) {
        let (mut clock, mut rng) = (Instant::now(), rand::thread_rng());
        loop {
            if clock.elapsed() > Duration::from_secs(rng.gen_range(start..end)) {
                clock = Instant::now();
                self.start_election().await;
            }
        }
    }

    async fn start_election(&mut self) {
        self.voted_for = self.id;
        self.votes_recieved.insert(self.id, true);

        for node in self.cluster.iter_mut() {
            let res = node
                .request_vote(Request::new(VoteRequest {
                    term: self.current_term + 1,
                    id: self.id as u64,
                    last_index: self.log.lock().await.len() as u64,
                    last_term: self.current_term,
                }))
                .await;
        }
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

    async fn join(&self, request: Request<Byte>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null {}))
    }
}
