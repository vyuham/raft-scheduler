use crate::raft_proto::{
    raft_server::Raft, Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
};
use std::{cmp::min, sync::Arc};
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
    pub commit_index: Arc<Mutex<u64>>,
    pub voted_for: u8,
    state: ServerState,
    pub id: u8,
    pub log: Arc<Mutex<Vec<(u64, T)>>>,
}

impl<T> RaftState<T> {
    pub fn new(id: u8, log: Arc<Mutex<Vec<(u64, T)>>>) -> Self {
        Self {
            current_term: 0,
            commit_index: Arc::new(Mutex::new(0)),
            voted_for: id,
            state: ServerState::Follower,
            id,
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
        request: tonic::Request<EntryRequest>,
    ) -> Result<tonic::Response<EntryReply>, tonic::Status> {
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
