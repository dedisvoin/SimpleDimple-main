use bevy::prelude::*;

/// Начальная позиция камеры.
pub fn initial_camera_position() -> Vec3 {
    Vec3::new(10.0, 15.0, 17.5)
}

/// Начальная позиция источника света.
pub fn initial_light_position() -> Vec3 {
    Vec3::new(8.0, 10.0, 6.0)
}

/// Список файлов материалов, загружаемых при старте.
pub const MATERIAL_FILES: [&str; 4] = [
    "assets/materials/red.txt",
    "assets/materials/blue.txt",
    "assets/materials/green.txt",
    "assets/materials/white.txt",
];

/// Описание размещения одного графического объекта на сцене.
pub struct ObjectLayout {
    /// Позиция объекта в глобальной системе координат.
    pub position: Vec3,
    /// Угол поворота вокруг оси Oy, в градусах.
    pub angle_deg: f32,
    /// Индекс материала в списке [`MATERIAL_FILES`].
    pub material_index: usize,
}

/// Размещение объектов сцены.
///
/// Белый — справа (+X), угол 180°.
/// Красный — слева (-X), угол 0°.
/// Синий — сзади (-Z), угол 90°.
/// Зелёный — спереди (+Z), угол 270°.
pub fn scene_layout() -> Vec<ObjectLayout> {
    vec![
        ObjectLayout {
            position: Vec3::new(4.0, 0.0, 0.0),
            angle_deg: 180.0,
            material_index: 3,
        },
        ObjectLayout {
            position: Vec3::new(-4.0, 0.0, 0.0),
            angle_deg: 0.0,
            material_index: 0,
        },
        ObjectLayout {
            position: Vec3::new(0.0, 0.0, -4.0),
            angle_deg: 90.0,
            material_index: 1,
        },
        ObjectLayout {
            position: Vec3::new(0.0, 0.0, 4.0),
            angle_deg: 270.0,
            material_index: 2,
        },
    ]
}
