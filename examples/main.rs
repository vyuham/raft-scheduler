use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use raft::scheduler::{RaftCommand, StateMachine};

#[derive(Debug, Copy, Clone)]
pub enum ExecTask {
    /// Generate memory representation for 3D model of world,
    /// create multiple units of execution for rendering
    Unroll = 0,
    /// Generate pixels from given units of execution and world
    Render = 1,
}

pub struct ExecUnit {
    pub task: ExecTask,
    pub data: Vec<u8>,
}

impl ExecUnit {
    /// Convert an ExecUnit into a stream of bytes
    fn as_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(self.task as u8);
        for data in self.data.iter() {
            vec.push(*data);
        }
        vec
    }

    /// Construct a ExecUnit from a stream of bytes
    fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            task: match bytes[0] {
                0 => ExecTask::Unroll,
                _ => ExecTask::Render,
            },
            data: bytes[1..].to_vec(),
        }
    }

    /// Execute the ExecUnit
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self.task {
            ExecTask::Unroll => {
                // Generate memory representation of the world
                // RTRCRS::create_world();
                // Create individual ExecUnits for each pixel
                for x in 0..400u16 {
                    for y in 0..225 {
                        let exec_unit = Self {
                            task: ExecTask::Render,
                            data: vec![(x >> 8) as u8, x as u8, y as u8],
                        };
                        // queue.push(exec_unit.as_bytes());
                    }
                }
            }
            ExecTask::Render => {
                let (i, j) = (
                    (self.data[0] as u16) << 8 | self.data[1] as u16,
                    self.data[2] as u16,
                );
                // RTRCRS::render(i, j);
            }
        }

        Ok(())
    }
}

struct Parallel {
    s: Arc<Mutex<StateMachine>>,
}

impl Parallel {
    pub fn new() -> Self {
        Self {
            s: Arc::new(Mutex::new(StateMachine::new())),
        }
    }

    pub async fn run(&self) {
        let cmd = RaftCommand::from_bytes(vec![0, 1, 2, 3, 4]);
        print!("{:#?}", self.s.lock().await.run_cmd(cmd));
    }
}

#[tokio::main]
async fn main() {
    Parallel::new().run(Config::new(0, 10, 15)).await;
}
