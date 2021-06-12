pub type RaftLog = Vec<(u64, Vec<u8>)>;

#[derive(Debug, Copy, Clone)]
pub enum RaftTask {
    Occupy = 1,
    Vacate = 0,
}

pub struct RaftCommand {
    /// Denotes task type
    pub task: RaftTask,
    /// Node ID, a 7bit number
    pub node: u8,
    /// Payload carried, an ExecTask
    pub data: Vec<u8>,
}

impl RaftCommand {
    pub fn as_bytes(&self) -> Vec<u8> {
        // Compress task and node detals into initial byte for messaging
        let task_node_byte = (self.task as u8) << 7 | self.node;
        let mut vec = vec![];
        vec.push(task_node_byte);
        // Push all data into message stream
        for data in self.data.iter() {
            vec.push(*data);
        }
        vec
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        // Generate RaftCommand from recieved byte stream
        Self {
            // Use task_bit from initial byte to determine task type
            task: match bytes[0] >> 7 {
                1 => RaftTask::Occupy,
                _ => RaftTask::Vacate,
            },
            // Remove task_bit and use rest of initial byte to determine node
            node: bytes[0] << 1 >> 1,
            // Use rest of byte stream as data payload
            data: bytes[1..].to_vec(),
        }
    }
}
