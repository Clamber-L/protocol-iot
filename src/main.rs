mod codec;
mod protocol;
mod server;
mod error;

use tokio::net::TcpListener;
use std::sync::Arc;

use codec::registry::ProtocolRegistry;
use protocol::gb26875::ProtocolA;
use server::config::ServerConfig;
use server::handler::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = ServerConfig::example(); // 或从文件加载

    // 创建协议注册表
    let mut registry = ProtocolRegistry::new();

    // 注册协议
    registry.register(8080, || ProtocolA);
    // registry.register(8081, || ProtocolB);

    // 为每个监听器启动任务
    let mut handles = vec![];
    println!("config:{:?}", config);

    for listener_config in config.listeners {
        let protocol_name = registry
            .get_protocol_name(listener_config.port)
            .expect("Protocol not registered for port");

        println!(
            "Starting listener on {}:{} with protocol {}",
            listener_config.bind_addr, listener_config.port, protocol_name
        );

        let addr = format!("{}:{}", listener_config.bind_addr, listener_config.port);
        let listener = TcpListener::bind(&addr).await?;

        let port = listener_config.port;

        // 根据不同协议启动不同的处理任务
        match port {
            8080 => {
                let protocol = Arc::new(ProtocolA);
                let handle = tokio::spawn(async move {
                    run_listener(listener, protocol).await.unwrap();
                });
                handles.push(handle);
            }
            // 8081 => {
            //     let protocol = Arc::new(ProtocolB);
            //     let handle = tokio::spawn(async move {
            //         run_listener(listener, protocol).await
            //     });
            //     handles.push(handle);
            // }
            _ => {
                eprintln!("Unknown port: {}", port);
            }
        }
    }

    // 等待所有监听器
    for handle in handles {
        handle.await?;
    }

    Ok(())
}

async fn run_listener<P>(
    listener: TcpListener,
    protocol: Arc<P>,
) -> Result<(), Box<dyn std::error::Error>>
where
    P: protocol::traits::Protocol,
{
    loop {
        let (socket, addr) = listener.accept().await?;
        let protocol = Arc::clone(&protocol);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, protocol, addr).await {
                eprintln!("[{}] Handler error: {}", addr, e);
            }
        });
    }
}