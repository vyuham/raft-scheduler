use std::{collections::HashMap, error::Error};
use tonic::Request;

use crate::{
    node::RaftData,
    raft_proto::{raft_client::RaftClient, VoteRequest},
};

/// Possible server states within a raft cluster
/// Follower: Can only respond to requests from nodes of cluster
/// Candidate: Can only request to be elected Leader of cluster
/// Leader: Operate until node failure, leads updates to state
pub enum ServerState {
    Follower,
    Candidate,
    Leader,
}

pub struct RaftDetails<T> {
    pub current_term: u64,
    pub commit_index: u64,
    pub voted_for: u8,
    pub votes_recieved: HashMap<u8, bool>,
    pub state: ServerState,
    pub id: u8,
    pub log: Vec<(u64, T)>,
    pub cluster: Vec<String>,
}

impl<T: RaftData + Sync + Send + 'static> RaftDetails<T> {
    pub fn new(id: u8, cluster: Vec<String>) -> Self {
        Self {
            current_term: 0,
            commit_index: 0,
            voted_for: id,
            votes_recieved: HashMap::new(),
            state: ServerState::Follower,
            id,
            log: vec![],
            cluster,
        }
    }

    pub async fn start_election(&mut self) -> Result<(), Box<dyn Error>> {
        self.voted_for = self.id;
        self.votes_recieved.insert(self.id, true);

        for node in self.cluster.iter_mut() {
            let res = RaftClient::connect(format!("http://{  }", node))
                .await?
                .request_vote(Request::new(VoteRequest {
                    term: self.current_term + 1,
                    id: self.id as u64,
                    last_index: self.log.len() as u64,
                    last_term: self.current_term,
                }))
                .await;
        }

        Ok(())
    }
}
