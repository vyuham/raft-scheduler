use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    raft::{RaftCommand, RaftLog, RaftTask},
    ScheduleError,
};

pub struct Scheduler {
    free_nodes: Vec<u8>,
    executing: HashMap<u8, Vec<u8>>,
}

impl Scheduler {
    pub fn default() -> Self {
        Self {
            executing: HashMap::new(),
            free_nodes: vec![],
        }
    }

    pub async fn updater(&mut self, input: &Vec<u8>) -> Result<(), ScheduleError> {
        Ok(self.run_cmd(RaftCommand::from_bytes(input)).await?)
    }

    async fn run_cmd(&mut self, cmd: RaftCommand) -> Result<(), ScheduleError> {
        match cmd.task {
            RaftTask::Occupy => {
                let node = cmd.node;
                if self.free_nodes.contains(&node) {
                    if !self.executing.contains_key(&node) {
                        self.free_nodes.retain(|i| i != &node);
                        self.executing.insert(node, cmd.data);
                        Ok(())
                    } else {
                        Err(ScheduleError("Couldn't schedule".to_string()))
                    }
                } else {
                    Err(ScheduleError("Couldn't occupy".to_string()))
                }
            }
            RaftTask::Vacate => {
                let node = cmd.node;
                if self.executing.contains_key(&node) {
                    if !self.free_nodes.contains(&node) {
                        self.free_nodes.push(node);
                        self.executing.remove(&node);
                        Ok(())
                    } else {
                        Err(ScheduleError("Couldn't vacate".to_string()))
                    }
                } else {
                    Err(ScheduleError("Couldn't unschedule".to_string()))
                }
            }
        }
    }
}

pub struct RaftStateMachine {
    log: Arc<Mutex<RaftLog>>,
    state: Arc<Mutex<Scheduler>>,
    curr_idx: usize,
}

impl RaftStateMachine {
    pub fn new(log: Arc<Mutex<RaftLog>>) -> Self {
        Self {
            log,
            state: Arc::new(Mutex::new(Scheduler::default())),
            curr_idx: 0,
        }
    }

    pub async fn updater(&mut self) {
        loop {
            let (log, mut state) = (self.log.lock().await, self.state.lock().await);
            if self.curr_idx + 1 < log.len() {
                let (_, ref data) = log[self.curr_idx];
                if let Err(e) = state.updater(data).await {
                    eprintln!("{}", e);
                } else {
                    self.curr_idx += 1;
                }
            }
        }
    }
}
