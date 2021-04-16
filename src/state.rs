use crate::raft_proto::{
    raft_server::Raft, Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub enum ServerState {
    Follower,
    Candidate,
    Leader,
}

pub struct RaftState<T> {
    term: u64,
    state: ServerState,
    pub schedule: Arc<Mutex<VecDeque<T>>>,
    pub executing: Arc<Mutex<HashMap<i8, T>>>,
    pub free_nodes: Arc<Mutex<Vec<i8>>>,
    pub log: Arc<Mutex<Vec<(u64, i8, T)>>>,
}

impl<T> RaftState<T> {
    pub fn new(
        schedule: Arc<Mutex<VecDeque<T>>>,
        executing: Arc<Mutex<HashMap<i8, T>>>,
        free_nodes: Arc<Mutex<Vec<i8>>>,
        log: Arc<Mutex<Vec<(u64, i8, T)>>>,
    ) -> Self {
        Self {
            term: 0,
            state: ServerState::Follower,
            schedule,
            executing,
            free_nodes,
            log,
        }
    }
}

#[tonic::async_trait]
impl<T: Sync + Send + 'static> Raft for RaftState<T> {
    async fn request_vote(
        &self,
        request: tonic::Request<VoteRequest>,
    ) -> Result<tonic::Response<VoteReply>, tonic::Status> {
        Ok(Response::new(VoteReply {
            term: self.term,
            grant: true,
        }))
    }

    async fn append_entries(
        &self,
        request: tonic::Request<EntryRequest>,
    ) -> Result<tonic::Response<EntryReply>, tonic::Status> {
        Ok(Response::new(EntryReply {
            term: self.term,
            success: true,
        }))
    }

    async fn join(&self, args: Request<Byte>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null {}))
    }
}
