use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{
    transport::{Channel, Server},
    Request, Response, Status,
};

use crate::{
    node::try_add,
    raft_proto::{
        raft_client::RaftClient,
        raft_server::{Raft, RaftServer},
        Byte, EntryReply, EntryRequest, Null, VoteReply, VoteRequest,
    },
};

pub struct RaftServerStub {
    clients: Arc<Mutex<HashMap<String, RaftClient<Channel>>>>,
}

impl RaftServerStub {
    pub fn new(clients: Arc<Mutex<HashMap<String, RaftClient<Channel>>>>) -> Self {
        Self { clients }
    }

    pub async fn start_server(self, addr: String) -> Result<(), Box<dyn std::error::Error>> {
        Server::builder()
            .add_service(RaftServer::new(self))
            .serve(addr.parse().unwrap())
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl Raft for RaftServerStub {
    async fn request_vote(
        &self,
        request: Request<VoteRequest>,
    ) -> Result<Response<VoteReply>, Status> {
        println!("{:?}", request.into_inner());

        Ok(Response::new(VoteReply {
            term: 0,
            grant: true,
        }))
    }

    async fn append_entries(
        &self,
        request: Request<EntryRequest>,
    ) -> Result<Response<EntryReply>, Status> {
        println!("{:?}", request.into_inner());

        Ok(Response::new(EntryReply {
            term: 0,
            success: true,
        }))
    }

    async fn join(&self, request: Request<Byte>) -> Result<Response<Null>, Status> {
        let req = request.into_inner().body;
        let addr = String::from_utf8(req).unwrap();
        println!("Adding {} to client list", addr);
        let mut clients = self.clients.lock().await;
        if let Some(_) = clients.keys().find(|&x| x == &addr) {
            Err(Status::already_exists("Client is already connected"))
        } else {
            clients.insert(addr.clone(), try_add(addr).await);
            Ok(Response::new(Null {}))
        }
    }
}
