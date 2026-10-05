// Модуль, содержащий функции для симуляции движения камеры в 3D пространстве на основе ввода с клавиатуры.

use crate::camera::CameraRig;
use bevy::prelude::*;

/// Скорость вращения камеры, градусы в секунду.
const ROTATION_SPEED_DEG_PER_SEC: f32 = 90.0;

/// Скорость приближения/удаления камеры, условных единиц в секунду.
const ZOOM_SPEED_UNITS_PER_SEC: f32 = 25.0;

pub fn simulate_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: ResMut<CameraRig>,
) {
    let delta_time = time.delta().as_secs_f32();

    // Защита от нулевого или отрицательного времени кадра.
    if delta_time <= 0.0 {
        return;
    }

    // Поворот влево/вправо.
    if keyboard.pressed(KeyCode::ArrowLeft) {
        camera.rotate_left_right(ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        camera.rotate_left_right(-ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }

    // Поворот вверх/вниз.
    if keyboard.pressed(KeyCode::ArrowUp) {
        camera.rotate_up_down(ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        camera.rotate_up_down(-ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }

    // Приближение/удаление: цифровой блок и основной ряд клавиатуры.
    let zoom_in = keyboard.pressed(KeyCode::NumpadAdd) || keyboard.pressed(KeyCode::Equal);
    let zoom_out = keyboard.pressed(KeyCode::NumpadSubtract) || keyboard.pressed(KeyCode::Minus);

    if zoom_in {
        // Приближение уменьшает радиус.
        camera.zoom_in_out(-ZOOM_SPEED_UNITS_PER_SEC * delta_time);
    }
    if zoom_out {
        // Удаление увеличивает радиус.
        camera.zoom_in_out(ZOOM_SPEED_UNITS_PER_SEC * delta_time);
    }
}

pub fn apply_camera_transform(
    camera: Res<CameraRig>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in &mut query {
        *transform = camera.transform();
    }
}
