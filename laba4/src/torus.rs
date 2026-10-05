// Модуль для управления тороидальными объектами сцены.

use bevy::prelude::*;

use crate::graphic_object::GraphicObject;

/// Скорость вращения объектов вокруг центра сцены (радианы в секунду).
const TORUS_ORBIT_SPEED: f32 = 0.5;

/// Палитра цветов для перекрашивания графических объектов.
#[derive(Resource)]
struct ColorPalette {
    colors: Vec<Color>,
    current_index: usize,
}

/// Плагин для управления тороидальными объектами сцены.
pub struct TorusPlugin;

impl Plugin for TorusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ColorPalette::new());
        app.add_systems(
            Update,
            (
                update_toruses, 
                update_torus_color_system, 
                torus_orbit_system
            )
        );
    }
}

impl ColorPalette {
    /// Конструктор для создания палитры цветов.
    pub fn new() -> Self {
        Self {
            colors: vec![
                Color::BLACK,
                Color::WHITE,
                Color::srgb(0.0, 0.0, 1.0),
                Color::srgb(1.0, 0.0, 0.0),
                Color::srgb(0.5, 0.0, 1.0),
            ],
            current_index: 0,
        }
    }

    /// Получаем текущий цвет.
    pub fn current_color(&self) -> Color {
        self.colors[self.current_index]
    }

    /// Функция для перехода к следующему цвету.
    pub fn next_color(&mut self) {
        self.current_index = (self.current_index + 1) % self.colors.len();
    }

    /// Функция для установки цвета по индексу.
    pub fn set_color(&mut self, index: usize) {
        self.current_index = index;
    }
}

/// Обновляет цвет всех графических объектов на основе текущего цвета палитры.
fn update_torus_color_system(
    palette: Res<ColorPalette>,
    mut materials: ResMut<Assets<StandardMaterial>>, // ресурс всех материалов
    query: Query<&MeshMaterial3d<StandardMaterial>, With<GraphicObject>>, // материалы торов
) {
    for material_handle in &query {
        // Пытаемся получить материал по его дескриптору (.0).
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = palette.current_color(); // устанавливаем цвет
        }
    }
}

/// Система вращения всех графических объектов вокруг центра координат.
///
/// Каждый кадр матрица модели объекта поворачивается вокруг начала координат:
/// торы движутся по окружности, а их «носики» остаются направленными в центр.
fn torus_orbit_system(time: Res<Time>, mut query: Query<&mut Transform, With<GraphicObject>>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(TORUS_ORBIT_SPEED * time.delta_secs()),
        );
    }
}

/// Система обработки нажатий клавиатуры для управления палитрой.
fn update_toruses(keyboard: Res<ButtonInput<KeyCode>>, mut palette: ResMut<ColorPalette>) {
    // Пробел переключает следующий цвет.
    if keyboard.just_pressed(KeyCode::Space) {
        palette.next_color();
    }

    // 1-5 переключают цвет по индексу.
    if keyboard.just_pressed(KeyCode::Digit1) {
        palette.set_color(0);
    }

    if keyboard.just_pressed(KeyCode::Digit2) {
        palette.set_color(1);
    }

    if keyboard.just_pressed(KeyCode::Digit3) {
        palette.set_color(2);
    }

    if keyboard.just_pressed(KeyCode::Digit4) {
        palette.set_color(3);
    }

    if keyboard.just_pressed(KeyCode::Digit5) {
        palette.set_color(4);
    }
}
