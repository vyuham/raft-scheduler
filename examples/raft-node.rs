use raft::node::{RaftNode, RaftData};

struct Exec {}

impl RaftData for Exec {
    fn as_bytes(&self) -> Vec<u8> {
        vec![]
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    RaftNode::<Exec>::start(
        0,
        "[::1]:50052".to_string(),
        vec![],
    ).await?.run(0, 10).await;

    Ok(())
}
