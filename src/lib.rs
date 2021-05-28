//! RaEx is a tool to help you build high performance compute clusters, with which you can run
//! computational tasks that would otherwise be incredibly inefficient on a single system.

pub mod config;
pub mod node;
pub mod raft;
pub mod state_machine;

mod raft_proto {
    tonic::include_proto!("raft");
}
