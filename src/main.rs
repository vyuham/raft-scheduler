use raft::RaftNode;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args[1].clone();
    println!("Starting {}", addr);

    let raft = RaftNode::new(addr).await;

    loop {}
}
