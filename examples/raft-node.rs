use raft::node::{RaftData, RaftNode};

struct Exec {}

impl RaftData for Exec {
    fn as_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from(_: Vec<u8>) -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    RaftNode::<Exec>::start(0, "[::1]:50052".to_string(), vec![])
        .await?
        .run(0u64, 10u64)
        .await;

    Ok(())
}
