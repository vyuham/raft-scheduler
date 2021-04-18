use crate::raft_proto::{
    raft_server::Raft, Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

/// Possible server states within a raft cluster
/// Follower: Can only respond to requests from nodes of cluster
/// Candidate: Can only request to be elected Leader of cluster
/// Leader: Operate until node failure, leads updates to state
pub enum ServerState {
    Follower,
    Candidate,
    Leader,
}

/// Datastructure to maintain state of cluster over Raft
pub struct RaftState<T> {
    pub current_term: u64,
    pub commit_index: u64,
    pub voted_for: u8,
    state: ServerState,
    pub id: u8,
    pub schedule: Arc<Mutex<VecDeque<T>>>,
    pub executing: Arc<Mutex<HashMap<u8, T>>>,
    pub free_nodes: Arc<Mutex<Vec<u8>>>,
    pub log: Arc<Mutex<Vec<(u64, u8, T)>>>,
}

impl<T> RaftState<T> {
    pub fn new(
        id: u8,
        schedule: Arc<Mutex<VecDeque<T>>>,
        executing: Arc<Mutex<HashMap<u8, T>>>,
        free_nodes: Arc<Mutex<Vec<u8>>>,
        log: Arc<Mutex<Vec<(u64, u8, T)>>>,
    ) -> Self {
        Self {
            current_term: 0,
            commit_index: 0,
            voted_for: id,
            state: ServerState::Follower,
            id,
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
            term: self.current_term,
            grant: true,
        }))
    }

    async fn append_entries(
        &self,
        request: tonic::Request<EntryRequest>,
    ) -> Result<tonic::Response<EntryReply>, tonic::Status> {
        Ok(Response::new(EntryReply {
            term: self.current_term,
            success: true,
        }))
    }

    async fn join(&self, args: Request<Byte>) -> Result<Response<Null>, Status> {
        Ok(Response::new(Null {}))
    }
}
