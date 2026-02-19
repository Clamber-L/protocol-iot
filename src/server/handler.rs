use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

use crate::protocol::traits::{Protocol, ProtocolMessage};
use crate::codec::dispatcher::ProtocolCodec;

pub async fn handle_connection<P>(
    socket: TcpStream,
    protocol: Arc<P>,
    peer_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>>
where
    P: Protocol,
{
    println!("[{}] New connection using protocol: {}", peer_addr, protocol.name());

    let codec = ProtocolCodec::new(protocol.as_ref());
    let mut framed = Framed::new(socket, codec);

    while let Some(result) = framed.next().await {
        match result {
            Ok(msg) => {
                println!("[{}] Received: {}", peer_addr, msg.summary());

                // 处理消息
                if let Some(response) = process_message(msg, protocol.as_ref()).await {
                    // framed.send(response).await?;
                }
            }
            Err(e) => {
                eprintln!("[{}] Decode error: {}", peer_addr, e);
                break;
            }
        }
    }

    println!("[{}] Connection closed", peer_addr);
    Ok(())
}

async fn process_message<P: Protocol>(
    msg: P::Message,
    _protocol: &P,
) -> Option<P::Message> {
    // 实现你的业务逻辑
    println!("Processing message: {}", msg.summary());

    // 返回响应（如果需要）
    None
}