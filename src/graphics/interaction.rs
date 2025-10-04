use bevy::{prelude::*, window::PrimaryWindow};
use crate::graphics::{*, entity::*};

#[derive(Resource)]
pub(super) struct CursorWorldPos(pub Option<Vec2>);

#[derive(Resource)]
pub(super) struct DragOperation {
    dragged_entity: Entity,
    start_cell: Entity,
}

#[derive(Resource)]
pub(super) struct ClickOperation {
    start_cell: Entity,
}

#[derive(Event)]
pub struct DragEvent {
    pub start_cell: Entity,
    pub end_cell: Entity,
}

#[derive(Event)]
pub struct ClickEvent {
    pub cell: Entity,
}

pub(super) fn get_cursor_world_pos(
    mut cursor_world_pos: ResMut<CursorWorldPos>,
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let primary_window = q_primary_window.single();
    let (main_camera, main_camera_transform) = q_camera.single().unwrap();
    // Get the cursor position in the world
    cursor_world_pos.0 = primary_window.unwrap()
        .cursor_position()
        .and_then(|cursor_pos| 
            main_camera.viewport_to_world_2d(main_camera_transform, cursor_pos).ok()
        );
}

// 根据光标位置获取光标所在格子，如果有多个则任意返回一个
fn cursor_cell(
    cursor_world_pos: Vec2,
    q_cell: Query<(&CellCom, Entity), With<CellCom>>,
) -> Option<Entity> {
    for (cell_com, entity) in q_cell.iter() {
        if cell_com.shape.contain(cursor_world_pos) {
            return Some(entity)
        }
    }
    return None
}

pub(super) fn just_click(
    mut commands: Commands,
    cursor_world_pos: Res<CursorWorldPos>,
    q_cell: Query<(&CellCom, Entity), With<CellCom>>,
) {
    // If the cursor is not within the primary window, skip this system
    let Some(cursor_world_pos) = cursor_world_pos.0 else {
        return;
    };

    let Some(cell) = cursor_cell(cursor_world_pos, q_cell) else {
        return;
    };

    let Ok((cell_com, _)) = q_cell.get(cell) else {
        error!("Click a cell with no CellCom");
        return;
    };

    if cell_com.dragable {
        commands.insert_resource(DragOperation {
            dragged_entity: cell_com.upper_piece,
            start_cell: cell,
        });
    } else if cell_com.clickable {
        commands.insert_resource(ClickOperation {
            start_cell: cell,
        });
    }
}

pub(super) fn just_release(
    mut commands: Commands,
    cursor_world_pos: Res<CursorWorldPos>,
    q_cell: Query<(&CellCom, Entity), With<CellCom>>,
    drag_operation: Option<Res<DragOperation>>,
    click_operation: Option<Res<ClickOperation>>,
    mut transforms: Query<&mut Transform>,
    mut ew_drag: EventWriter<DragEvent>,
    mut ew_click: EventWriter<ClickEvent>,
) {
    if let Some(drag_operation) = drag_operation {
        let entity = drag_operation.dragged_entity;
        let start_cell = drag_operation.start_cell;

        if let Some(cursor_world_pos) = cursor_world_pos.0 {
            if let Some(end_cell) = cursor_cell(cursor_world_pos, q_cell) {
                if start_cell == end_cell {
                    ew_click.write(ClickEvent { cell: start_cell });
                } else {
                    ew_drag.write(DragEvent { start_cell, end_cell });
                }
            }
        }

        let start_center = q_cell.get(start_cell).unwrap().0.shape.center();

        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.translation.x = start_center.x;
            transform.translation.y = start_center.y;
        }

        commands.remove_resource::<DragOperation>();
    }

    if let Some(click_operation) = click_operation {
        let start_cell = click_operation.start_cell;

        if let Some(cursor_world_pos) = cursor_world_pos.0 {
            if let Some(end_cell) = cursor_cell(cursor_world_pos, q_cell) {
                if start_cell == end_cell {
                    ew_click.write(ClickEvent { cell: start_cell });
                }
            }
        }

        commands.remove_resource::<ClickOperation>();
    }
}

pub(super) fn drag(
    drag_operation: Res<DragOperation>,
    cursor_world_pos: Res<CursorWorldPos>,
    mut transforms: Query<&mut Transform>,
) {
    let Some(cursor_world_pos) = cursor_world_pos.0 else {
        return;
    };

    let entity = drag_operation.dragged_entity;

    if let Ok(mut transform) = transforms.get_mut(entity) {
        transform.translation.x = cursor_world_pos.x;
        transform.translation.y = cursor_world_pos.y;
    }
}