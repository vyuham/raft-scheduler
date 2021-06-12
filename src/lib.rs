use std::{error::Error, fmt};

mod node;
mod raft;
mod state;

mod raft_proto {
    tonic::include_proto!("raft");
}

#[derive(Debug)]
pub struct ScheduleError(String);

impl Error for ScheduleError {}

impl fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scheduling error: {}", self.0)
    }
}
