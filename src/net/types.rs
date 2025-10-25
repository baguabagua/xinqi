use bevy::prelude::*;
use std::net::SocketAddr;

// 网络状态
#[derive(Resource, Default, Debug)]
pub enum NetState {
    #[default]
    Disconnected,
    Listening(SocketAddr, tokio::sync::oneshot::Sender<()>),
    Connected(SocketAddr, tokio::sync::oneshot::Sender<()>),
}

// 网络消息事件
#[derive(Event)]
pub struct ReceiveNetMsgEvent {
    pub message: String,
}

// 发送消息事件
#[derive(Event)]
pub struct SendNetMsgEvent {
    pub message: String,
}

// 网络命令
#[derive(Event)]
pub enum NetCommand {
    Listen(String),
    Connect(String),
    Disconnect,
}

// 内部网络连接
#[derive(Resource)]
pub struct NetConnection {
    pub tx: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    pub incoming_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
}