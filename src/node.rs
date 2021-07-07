use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request};

use crate::raft_proto::{raft_client::RaftClient, Byte};

pub mod client;
pub mod server;

use client::RaftClientStub;
use server::RaftServerStub;

pub struct RaftNodeInner {}

pub struct RaftNode {
    pub client: RaftClientStub,
}

impl RaftNode {
    pub async fn new(self_addr: String, client_addrs: Vec<String>) -> Self {
        let mut clients = HashMap::new();
        for addr in client_addrs {
            clients.insert(addr.clone(), try_add(addr).await);
        }

        for (_, c) in clients.iter_mut() {
            match c
                .join(Request::new(Byte {
                    body: self_addr.clone().into_bytes(),
                }))
                .await
            {
                Ok(_) => println!("r"),
                Err(_) => println!("Error joining"),
            }
        }

        let clients = Arc::new(Mutex::new(clients));
        let clients_clone = clients.clone();

        tokio::spawn(async move {
            RaftServerStub::new(clients_clone)
                .start_server(self_addr)
                .await;
        });

        Self {
            client: RaftClientStub::new(clients).await,
        }
    }
}

pub async fn try_add(addr: String) -> RaftClient<Channel> {
    loop {
        match RaftClient::connect(format!("http://{}", addr)).await {
            Ok(c) => {
                println!("connected to {}", addr);
                return c;
            }
            Err(_) => continue,
        }
    }
}
