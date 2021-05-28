use raft::{
    config::Config,
    node::{RaftData, RaftNode},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    RaftNode::start(0, "[::1]:50052".to_string(), vec![])
        .await?
        .run(Config::new(0, 10, 15))
        .await;

    Ok(())
}
