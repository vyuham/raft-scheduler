//! RaEx is a tool to help you build high performance compute clusters, with which you can run
//! computational tasks that would otherwise be incredibly inefficient on a single system.

use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    sync::Arc,
};
use tokio::sync::Mutex;
use tonic::{
    transport::{Channel, Server},
    Request, Response, Status,
};

mod raft_proto {
    tonic::include_proto!("raft");
}

use raft_proto::{
    raft_client::RaftClient,
    raft_server::{Raft, RaftServer},
    Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
};

struct RaftState<T> {
    term: u64,
    state: ServerState,
    pub schedule: Arc<Mutex<VecDeque<T>>>,
    pub executing: Arc<Mutex<HashMap<i8, T>>>,
    pub free_nodes: Arc<Mutex<Vec<i8>>>,
    pub log: Arc<Mutex<Vec<(u64, i8, T)>>>,
}

pub enum ServerState {
    Follower,
    Candidate,
    Leader,
}

pub struct Consensus<T> {
    raft: RaftState<T>,
    clients: Vec<RaftClient<Channel>>,
}

impl<T: Sync + Send + 'static> Consensus<T> {
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
        let raft = RaftState {
            term: 0,
            state: ServerState::Follower,
            schedule: schedule.clone(),
            executing: executing.clone(),
            free_nodes: free_nodes.clone(),
            log: log.clone(),
        };

        // Server runs on a background thread and handles calls to the node
        tokio::spawn(async move {
            Server::builder()
                .add_service(RaftServer::new(raft))
                .serve(local_addr.parse().unwrap())
                .await;
        });

        Ok(Self {
            raft: RaftState {
                term: 0,
                state: ServerState::Follower,
                schedule,
                executing,
                free_nodes,
                log,
            },
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

#[tonic::async_trait]
impl<T: Sync + Send + 'static> Raft for RaftState<T> {
    async fn request_vote(&self, request:tonic::Request<VoteRequest>) ->Result<tonic::Response<VoteReply>,tonic::Status> {
        Ok(Response::new(VoteReply {
            term: self.term,
            grant: true,
        }))
    }

    async fn append_entries(&self, request:tonic::Request<EntryRequest>) ->Result<tonic::Response<EntryReply>,tonic::Status> {
        Ok(Response::new(EntryReply {
            term: self.term,
            success: true,
        }))
    }

    async fn join(&self, args: Request<Byte>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null {}))
    }
}
