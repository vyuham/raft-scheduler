mod raft_proto {
    tonic::include_proto!("raft");
}

mod node;

pub use node::RaftNode;
