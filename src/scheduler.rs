use std::{
    collections::HashMap, error::Error, fmt
};

#[derive(Debug, Copy, Clone)]
pub enum RaftTask {
    Schedule = 1,
    UnSchedule = 2,
    Occupy = 3,
    Vacate = 4,
}

pub struct RaftCommand {
    pub task: RaftTask,
    pub node: u8,
    pub data: Vec<u8>,
}

impl RaftCommand {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(self.task as u8);
        vec.push(self.node);
        for data in self.data.iter() {
            vec.push(*data);
        }
        vec
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            task: match bytes[0] {
                1 => RaftTask::Schedule,
                2 => RaftTask::UnSchedule,
                3 => RaftTask::Occupy,
                _ => RaftTask::Vacate,
            },
            node: bytes[1],
            data: bytes[2..].to_vec(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ExecTask {
    UnRoll = 1,
    Render = 2,
    RollUp = 3,
}

pub struct ExecUnit {
    pub task: ExecTask,
    pub data: Vec<u8>,
}

impl ExecUnit {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(self.task as u8);
        for data in self.data.iter() {
            vec.push(*data);
        }
        vec
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            task: match bytes[0] {
                1 => ExecTask::UnRoll,
                2 => ExecTask::Render,
                _ => ExecTask::RollUp,
            },
            data: bytes[1..].to_vec(),
        }
    }
}

pub struct StateMachine {
    executing: HashMap<u8, ExecUnit>,
    free_nodes: Vec<u8>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            executing: HashMap::new(),
            free_nodes: vec![],
        }
    }

    pub fn run_cmd(&mut self, cmd: RaftCommand) -> Result<(), ScheduleError> {
        match cmd.task {
            RaftTask::Schedule => {
                let node = cmd.data[0];
                let data = ExecUnit::from_bytes(cmd.data[1..].to_vec());
                if self.executing.contains_key(&node) {
                    self.executing.insert(node, data);
                    Ok(())
                } else {
                    Err(ScheduleError("Couldn't schedule".to_string()))
                }
            },
            RaftTask::UnSchedule => {
                let node = cmd.data[0];
                if !self.executing.contains_key(&node) {
                    self.executing.remove(&node);
                    Ok(())
                } else {
                    Err(ScheduleError("Couldn't unschedule".to_string()))
                }
            },
            RaftTask::Occupy => {
                let node = cmd.data[0];
                if self.free_nodes.contains(&node) {
                    self.free_nodes.retain(|&i| i != node);
                    Ok(())
                } else {
                    Err(ScheduleError("Couldn't occupy".to_string()))
                }
            },
            RaftTask::Vacate => {
                let node = cmd.data[0];
                if !self.free_nodes.contains(&node) {
                    self.free_nodes.push(node);
                    Ok(())
                } else {
                    Err(ScheduleError("Couldn't vacate".to_string()))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ScheduleError(String);

impl Error for ScheduleError {}

impl fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scheduling error: {}", self.0)
    }
}