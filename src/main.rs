use raft::RaftNode;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args[1].clone();
    println!("Starting {}", addr);

    let mut raft = RaftNode::new(addr, args[2..].to_vec()).await;

    loop {
        raft.client.request().await;
        raft.client.append().await;
    }
}
