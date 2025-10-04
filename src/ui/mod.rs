use bevy::{prelude::*, render::view::RenderLayers};
use bevy_egui::{EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext};

use crate::{tree::game_tree_event::{DeleteVariationEvent, MoveToNodeEvent}, ui::{ui_game_tree::*, ui_hequn::*, ui_menu::*, ui_sl::*}};

pub mod ui_menu;
pub mod ui_sl;
pub mod ui_game_tree;
pub mod ui_hequn;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }

        app.add_event::<MoveToNodeEvent>();
        app.add_event::<DeleteVariationEvent>();
        app.init_resource::<UiMenuState>();
        app.init_resource::<UiSlState>();
        app.add_systems(Startup, ui_setup);
        app.add_systems(
            EguiPrimaryContextPass,
            (
                ui_menu,
                ui_game_tree,
                ui_sl,
                ui_hequn,
            )
        );
    }
}

fn ui_setup(
    mut commands: Commands, 
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;

    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        RenderLayers::none(),
        Camera {
            order: 1,
            ..default()
        },
    ));
}