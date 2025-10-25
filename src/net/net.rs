use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}};
use crate::net::types::*;
use std::{net::SocketAddr};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub fn handle_net_commands(
    mut _commands: Commands,
    mut net_commands: EventReader<NetCommand>,
    runtime: Res<TokioTasksRuntime>,
    mut net_state: ResMut<NetState>,
) {
    for command in net_commands.read() {
        match command {
            NetCommand::Listen(addr) => {
                match *net_state {
                    NetState::Disconnected => {},
                    _ => { continue; }
                }

                let addr = addr.clone();
                runtime.spawn_background_task(move |ctx| async move {
                    if let Err(e) = listen_server(addr, ctx).await {
                        error!("Server error: {}", e);
                    }
                });
            }
            
            NetCommand::Connect(addr) => {
                match *net_state {
                    NetState::Disconnected => {},
                    _ => { continue; }
                }

                let addr = addr.clone();
                runtime.spawn_background_task(move |ctx| async move {
                    if let Err(e) = connect_client(addr, ctx).await {
                        eprintln!("Connection error: {}", e);
                    }
                });
            }
            
            NetCommand::Disconnect => {
                // take 获取所有权的同时会将 net_state 自动置为默认值，也就是 Disconnected
                match std::mem::take(&mut *net_state) {
                    NetState::Disconnected => {},
                    NetState::Listening(socket_addr, sender) => {
                        println!("Try to cancel server listening to {}", socket_addr);
                        let _ = sender.send(());
                    },
                    NetState::Connected(socket_addr, sender) => {
                        println!("Try to disconnect with {}", socket_addr);
                        let _ = sender.send(());
                    },
                }
            }
        }
    }
}

async fn listen_server(
    addr: String,
    mut ctx: bevy_tokio_tasks::TaskContext,
) -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let local_addr = listener.local_addr()?;
    
    info!("Server listening on {}", local_addr);

    let (drop_tx, drop_rx) = tokio::sync::oneshot::channel();
    
    ctx.run_on_main_thread(move |main_ctx| {
        *main_ctx.world.resource_mut::<NetState>() = NetState::Listening(local_addr, drop_tx);
    }).await;
    
    tokio::select! {
        result = listener.accept() => {
            match result {
                Ok((socket, peer_addr)) => {
                    handle_connection(socket, peer_addr, ctx).await?;
                }
                Err(e) => {
                    return Err(Box::new(e) as Error);
                }
            }
        }
        // 监听一次性通道
        _ = drop_rx => {
            println!("Cancel server listening");
            // 这里不需要手动 drop(listener)
        }
    }

    Ok(())
}

async fn connect_client(
    addr: String,
    ctx: bevy_tokio_tasks::TaskContext,
) -> Result<(), Error> {
    let socket = tokio::net::TcpStream::connect(&addr).await?;
    let peer_addr = socket.peer_addr()?;

    handle_connection(socket, peer_addr, ctx).await?;
    Ok(())
}

async fn handle_connection(
    socket: tokio::net::TcpStream,
    peer_addr: SocketAddr,
    mut ctx: bevy_tokio_tasks::TaskContext,
) -> Result<(), Error> {
    info!("Client to: {}", peer_addr);
    let (drop_tx, drop_rx) = tokio::sync::oneshot::channel();
    ctx.run_on_main_thread(move |main_ctx| {
        *main_ctx.world.resource_mut::<NetState>() = NetState::Connected(peer_addr, drop_tx);
    }).await;

    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    
    // 创建消息通道
    let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel();
    let (outgoing_tx, mut outgoing_rx) = tokio::sync::mpsc::unbounded_channel();
    
    // 在主线程注册连接
    ctx.run_on_main_thread(move |main_ctx| {
        main_ctx.world.insert_resource(NetConnection {
            tx: Some(outgoing_tx),
            incoming_rx,
        });
    }).await;
    
    // 接收消息任务
    let read_handle = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let msg = line.trim().to_string();
                    if !msg.is_empty() {
                        let _ = incoming_tx.send(msg);
                    }
                }
                Err(e) => { 
                    error!("Read error: {}", e);
                    break;
                },
            }
        }
    });
    
    // 发送消息任务
    let write_handle = tokio::spawn(async move {
        while let Some(msg) = outgoing_rx.recv().await {
            if let Err(e) = writer.write_all(msg.as_bytes()).await {
                error!("Write error: {}", e);
                break;
            }
            if let Err(e) = writer.write_all(b"\n").await {
                error!("Write newline error: {}", e);
                break;
            }
            if let Err(e) = writer.flush().await {
                error!("Flush error: {}", e);
                break;
            }
        }
    });

    // 等待两个任务完成（任意一个结束就退出）
    tokio::select! {
         _ = read_handle => {
            info!("Read task finished, connection closed");
        },
        _ = write_handle => {
            info!("Write task finished, connection closed");
        },
        _ = drop_rx => {
            info!("Disconnection requested by user");
        },
    }

    ctx.run_on_main_thread(|main_ctx| {
        *main_ctx.world.resource_mut::<NetState>() = NetState::Disconnected;
        main_ctx.world.remove_resource::<NetConnection>();
    }).await;

    Ok(())
}

pub fn process_incoming_messages(
    net_connection: Option<ResMut<NetConnection>>,
    mut receive_events: EventWriter<ReceiveNetMsgEvent>,
) {
    if let Some(mut connection) = net_connection {
        while let Ok(message) = connection.incoming_rx.try_recv() {
            receive_events.write(ReceiveNetMsgEvent { message });
        }
    }
}

pub fn handle_outgoing_messages(
    mut send_events: EventReader<SendNetMsgEvent>,
    net_connection: Option<ResMut<NetConnection>>,
) {
    if let Some(connection) = net_connection {
        for event in send_events.read() {
            if let Some(tx) = &connection.tx {
                let _ = tx.send(event.message.clone());
            }
        }
    }
}