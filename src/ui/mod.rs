use bevy::{prelude::*, render::view::RenderLayers};
use bevy_egui::{EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext};

use crate::{hequn::game::HequnGame, tree::game_tree_event::{DeleteVariationEvent, MoveToNodeEvent}, ui::{
        ui_game_tree::*, ui_hequn::*, ui_menu::*, ui_sl::*, ui_xingxiang::*, ui_zhandi::*
    }, xingxiang::game::XingxiangGame, zhandi::game::ZhandiGame
};

pub mod ui_menu;
pub mod ui_sl;
pub mod ui_game_tree;
pub mod ui_hequn;
pub mod ui_zhandi;
pub mod ui_xingxiang;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }

        app.add_event::<MoveToNodeEvent>();
        app.add_event::<DeleteVariationEvent>();
        app.init_resource::<UiMenuState>();
        app.init_resource::<UiSlState::<HequnGame>>();
        app.init_resource::<UiSlState::<ZhandiGame>>();
        app.init_resource::<UiSlState::<XingxiangGame>>();
        app.add_systems(Startup, ui_setup);
        app.add_systems(
            EguiPrimaryContextPass,
            (
                ui_menu,
                ui_game_tree::<HequnGame>,
                ui_game_tree::<ZhandiGame>,
                ui_game_tree::<XingxiangGame>,
                ui_sl::<HequnGame>,
                ui_sl::<ZhandiGame>,
                ui_sl::<XingxiangGame>,
                ui_hequn,
                ui_zhandi,
                ui_xingxiang,
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