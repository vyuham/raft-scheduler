use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::messages::Messages;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Serde error")]
    Serde(#[from] serde_json::Error),
    #[error("Stopped receiving data over channel")]
    Stopped,
}

pub struct Network {
    stream: Framed<TcpStream, LengthDelimitedCodec>,
}

impl Network {
    fn new(stream: Framed<TcpStream, LengthDelimitedCodec>) -> Network {
        Network { stream }
    }

    async fn get(&mut self) -> Result<Messages, Error> {
        let bytes = self.stream.next().await.ok_or(Error::Stopped)??;
        let msg = serde_json::from_slice(&bytes).unwrap();
        
        Ok(msg)
    }

    async fn put(&mut self, msg: Messages) -> Result<(), Error> {
        let bytes = Bytes::from(serde_json::to_vec(&msg)?);
        self.stream.send(bytes).await?;

        Ok(())
    }
}
