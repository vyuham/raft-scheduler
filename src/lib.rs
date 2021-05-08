//! RaEx is a tool to help you build high performance compute clusters, with which you can run
//! computational tasks that would otherwise be incredibly inefficient on a single system.

mod node;

pub use node::{RaftNode, RaftData};

mod raft_proto {
    tonic::include_proto!("raft");
}

pub use raft_proto::raft_client::RaftClient;
