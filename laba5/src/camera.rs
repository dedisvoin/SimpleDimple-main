use bevy::prelude::*;

const MIN_ANGLE_Y_DEG: f32 = 0.0;
const MAX_ANGLE_Y_DEG: f32 = 89.0;

const MIN_RADIUS: f32 = 5.0;
const MAX_RADIUS: f32 = 100.0;

#[derive(Resource)]
pub struct CameraRig {
    /// Расстояние от камеры до точки наблюдения.
    radius: f32,
    /// Горизонтальный угол поворота камеры вокруг оси Oy, в градусах.
    angle_x_deg: f32,
    /// Угол возвышения камеры над горизонтальной плоскостью, в градусах.
    angle_y_deg: f32,
    /// Текущая позиция камеры в глобальной системе координат.
    position: Vec3,
}

impl CameraRig {
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

    // установка и получение позиции камеры
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.radius = position.length();
        self.angle_x_deg = position.x.atan2(position.z).to_degrees();
        self.angle_y_deg = (position.y / self.radius).asin().to_degrees();
    }

    // методы для перемещения камеры
    pub fn rotate_left_right(&mut self, degrees: f32) {
        self.angle_x_deg = (self.angle_x_deg + degrees).rem_euclid(360.0);
        self.recalculate_position();
    }
    pub fn rotate_up_down(&mut self, degrees: f32) {
        self.angle_y_deg = (self.angle_y_deg + degrees).clamp(MIN_ANGLE_Y_DEG, MAX_ANGLE_Y_DEG);
        self.recalculate_position();
    }
    pub fn zoom_in_out(&mut self, distance: f32) {
        self.radius = (self.radius + distance).clamp(MIN_RADIUS, MAX_RADIUS);
        self.recalculate_position();
    }

    // устанавливаем камеру в нужную позицию
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position).looking_at(Vec3::ZERO, Vec3::Y)
    }

    // пересчёт позиции камеры после изменения сферических параметров
    fn recalculate_position(&mut self) {
        let angle_x_rad = self.angle_x_deg.to_radians();
        let angle_y_rad = self.angle_y_deg.to_radians();
        let x = self.radius * angle_y_rad.cos() * angle_x_rad.sin();
        let y = self.radius * angle_y_rad.sin();
        let z = self.radius * angle_y_rad.cos() * angle_x_rad.cos();
        self.position = Vec3::new(x, y, z);
    }
}
