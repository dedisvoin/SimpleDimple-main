//! Направленная камера, вращающаяся вокруг центра сцены.

use bevy::prelude::*;

use crate::{config::CAMERA_POSITION, schedule::GameSet};

/// Минимальный и максимальный угол возвышения камеры из реализации ЛР №6.
const MIN_ANGLE_Y_DEG: f32 = 0.0;
const MAX_ANGLE_Y_DEG: f32 = 89.0;

/// Минимальное и максимальное расстояние до центра сцены.
const MIN_RADIUS: f32 = 5.0;
const MAX_RADIUS: f32 = 100.0;

/// Скорость вращения камеры, градусов в секунду.
const ROTATION_SPEED_DEG_PER_SEC: f32 = 90.0;

/// Скорость приближения и удаления камеры, единиц в секунду.
const ZOOM_SPEED_UNITS_PER_SEC: f32 = 25.0;

/// Сферические параметры направленной камеры и её декартова позиция.
///
/// Камера всегда смотрит в центр сцены. Состояние меняется через методы,
/// поддерживающие согласованность позиции, углов и расстояния.
#[derive(Resource, Debug, Clone, Copy)]
pub struct CameraRig {
    radius: f32,
    angle_x_deg: f32,
    angle_y_deg: f32,
    position: Vec3,
}

impl CameraRig {
    /// Создаёт камеру в заданной позиции относительно центра сцены.
    pub fn new(position: Vec3) -> Self {
        let mut camera = Self {
            radius: 0.0,
            angle_x_deg: 0.0,
            angle_y_deg: 0.0,
            position: Vec3::ZERO,
        };
        camera.set_position(position);
        camera
    }

    /// Устанавливает позицию камеры и пересчитывает сферические параметры.
    pub fn set_position(&mut self, position: Vec3) {
        let length = position.length();
        if length <= f32::EPSILON {
            self.radius = MIN_RADIUS;
            self.angle_x_deg = 0.0;
            self.angle_y_deg = MIN_ANGLE_Y_DEG;
            self.recalculate_position();
            return;
        }

        self.radius = length.clamp(MIN_RADIUS, MAX_RADIUS);
        let vertical_ratio = (position.y / length).clamp(-1.0, 1.0);
        self.angle_y_deg = vertical_ratio
            .asin()
            .to_degrees()
            .clamp(MIN_ANGLE_Y_DEG, MAX_ANGLE_Y_DEG);

        let horizontal_length = Vec3::new(position.x, 0.0, position.z).length();
        self.angle_x_deg = if horizontal_length <= f32::EPSILON {
            0.0
        } else {
            position.x.atan2(position.z).to_degrees().rem_euclid(360.0)
        };

        self.recalculate_position();
    }

    /// Возвращает позицию камеры в мировых координатах.
    #[allow(dead_code)]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Вращает камеру вокруг вертикальной оси. Положительное значение — влево.
    pub fn rotate_left_right(&mut self, degrees: f32) {
        self.angle_x_deg = (self.angle_x_deg + degrees).rem_euclid(360.0);
        self.recalculate_position();
    }

    /// Меняет угол возвышения камеры, сохраняя его в диапазоне 0°–89°.
    pub fn rotate_up_down(&mut self, degrees: f32) {
        self.angle_y_deg = (self.angle_y_deg + degrees).clamp(MIN_ANGLE_Y_DEG, MAX_ANGLE_Y_DEG);
        self.recalculate_position();
    }

    /// Меняет расстояние до центра сцены, сохраняя его в диапазоне 5–100.
    /// Положительное значение удаляет камеру, отрицательное приближает.
    pub fn zoom_in_out(&mut self, distance: f32) {
        self.radius = (self.radius + distance).clamp(MIN_RADIUS, MAX_RADIUS);
        self.recalculate_position();
    }

    /// Возвращает трансформацию камеры, направленной в центр сцены.
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position).looking_at(Vec3::ZERO, Vec3::Y)
    }

    /// Пересчитывает декартову позицию из сферических параметров.
    fn recalculate_position(&mut self) {
        let angle_x_rad = self.angle_x_deg.to_radians();
        let angle_y_rad = self.angle_y_deg.to_radians();
        let horizontal_radius = self.radius * angle_y_rad.cos();

        self.position = Vec3::new(
            horizontal_radius * angle_x_rad.sin(),
            self.radius * angle_y_rad.sin(),
            horizontal_radius * angle_x_rad.cos(),
        );
    }
}

/// Регистрирует состояние камеры и системы управления/обновления.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraRig::new(CAMERA_POSITION))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, simulate_camera.in_set(GameSet::Input))
            .add_systems(Update, apply_camera_transform.in_set(GameSet::Render));
    }
}

/// Создаёт сущность 3D-камеры с начальной трансформацией из `CameraRig`.
fn spawn_camera(mut commands: Commands, camera: Res<CameraRig>) {
    commands.spawn((Camera3d::default(), camera.transform()));
}

/// Обновляет состояние камеры по удерживаемым клавишам с учётом времени кадра.
fn simulate_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: ResMut<CameraRig>,
) {
    let delta_time = time.delta_secs();
    if delta_time <= 0.0 {
        return;
    }

    if keyboard.pressed(KeyCode::ArrowLeft) {
        camera.rotate_left_right(ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        camera.rotate_left_right(-ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        camera.rotate_up_down(ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        camera.rotate_up_down(-ROTATION_SPEED_DEG_PER_SEC * delta_time);
    }

    let zoom_in = keyboard.pressed(KeyCode::NumpadAdd) || keyboard.pressed(KeyCode::Equal);
    let zoom_out = keyboard.pressed(KeyCode::NumpadSubtract) || keyboard.pressed(KeyCode::Minus);

    if zoom_in {
        camera.zoom_in_out(-ZOOM_SPEED_UNITS_PER_SEC * delta_time);
    }
    if zoom_out {
        camera.zoom_in_out(ZOOM_SPEED_UNITS_PER_SEC * delta_time);
    }
}

/// Переносит актуальную трансформацию из ресурса в сущность камеры.
fn apply_camera_transform(
    camera: Res<CameraRig>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    if !camera.is_changed() {
        return;
    }
    for mut transform in &mut query {
        *transform = camera.transform();
    }
}
