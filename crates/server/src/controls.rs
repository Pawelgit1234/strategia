use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

use crate::config::CAMERA_SPEED;

pub fn move_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    camera.translation +=
        (direction.normalize_or_zero() * CAMERA_SPEED * time.delta_secs()).extend(0.0);
}


pub fn zoom_camera(
    mut camera: Single<&mut Projection, With<Camera2d>>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    let Projection::Orthographic(projection) = &mut **camera else {
        return;
    };

    projection.scale *= 1.0 - scroll.delta.y * 0.1;
    projection.scale = projection.scale.clamp(0.2, 10.0);
}
