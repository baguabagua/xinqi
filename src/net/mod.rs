use bevy::prelude::*;

use crate::net::{message::*, net::*, types::*};

mod types;
mod net;
pub mod message;

pub use crate::net::types::{NetState, NetCommand, ReceiveNetMsgEvent, SendNetMsgEvent};

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_tokio_tasks::TokioTasksPlugin>() {
            app.add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default());
        }
        app.init_resource::<NetState>();
        app.add_event::<NetCommand>();
        app.add_event::<ReceiveNetMsgEvent>();
        app.add_event::<SendNetMsgEvent>();
        app.add_event::<ReceiveRemoteStep>();
        app.add_event::<SendRemoteStep>();
        app.add_systems(
            Update, 
            (
                handle_net_commands,
                process_incoming_messages,
                handle_outgoing_messages,
            )
        );
    }
}