// main.rs
use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy::window::{PrimaryWindow, Window};
use bevy_ecs_tilemap::prelude::*;

mod tilemap;

const CAMERA_PAN_SPEED: f32 = 200.0;
const CAMERA_PAN_PROXIMITY: f32 = 50.0;
const ZOOM_FACTOR: f32 = 0.1;
const TRACKPAD_ZOOM_SENSITIVITY: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, edge_based_camera_pan)
        .add_systems(Update, scroll_wheel_zoom)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup orthographic camera with initial zoom
    commands.spawn((
        Camera2d::default(),
        Transform::from_scale(Vec3::splat(0.7)), // Slightly zoomed in by default
    ));

    // Create our tilemap
    tilemap::create_tilemap(&mut commands, asset_server);
}

fn edge_based_camera_pan(
    mouse_pos_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let window = mouse_pos_query.single();
    if let Some(cursor_pos) = window.cursor_position() {
        let win_width = window.width();
        let win_height = window.height();
        let mut pan_direction = Vec3::ZERO;

        if cursor_pos.x <= CAMERA_PAN_PROXIMITY {
            pan_direction.x -= 1.0;
        }
        if cursor_pos.x >= win_width - CAMERA_PAN_PROXIMITY {
            pan_direction.x += 1.0;
        }
        if cursor_pos.y <= CAMERA_PAN_PROXIMITY {
            pan_direction.y += 1.0;
        }
        if cursor_pos.y >= win_height - CAMERA_PAN_PROXIMITY {
            pan_direction.y -= 1.0;
        }

        if pan_direction.length_squared() > 0.0 {
            let pan_speed = CAMERA_PAN_SPEED * time.delta_secs();
            for mut transform in camera_query.iter_mut() {
                transform.translation += pan_direction.normalize() * pan_speed;
            }
        }
    }
}

fn scroll_wheel_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut delta = 0.0;

    for ev in scroll_evr.read() {
        delta += ev.y;
    }

    if delta.abs() > f32::EPSILON {
        let zoom_amount = if delta > 0.0 {
            1.0 - ZOOM_FACTOR * TRACKPAD_ZOOM_SENSITIVITY // Zoom in
        } else {
            1.0 + ZOOM_FACTOR * TRACKPAD_ZOOM_SENSITIVITY // Zoom out
        };
        camera_transform.scale *= zoom_amount;
    }
}