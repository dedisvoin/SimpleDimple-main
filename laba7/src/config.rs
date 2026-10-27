use bevy::prelude::*;

/// Скорость перемещения игрока, клеток в секунду.
pub const PLAYER_SPEED: f32 = 3.0;

/// Скорость перемещения игрока, толкающего лёгкий объект, клеток в секунду.
pub const PUSH_SPEED: f32 = 2.0;

/// Размер одной ячейки карты в мировых единицах.
pub const CELL_SIZE: f32 = 1.0;

/// Высота центра объекта над плоскостью основания.
pub const OBJECT_HEIGHT: f32 = 0.5;

/// Начальная позиция камеры.
pub const CAMERA_POSITION: Vec3 = Vec3::new(18.0, 22.0, 26.0);

/// Начальная позиция источника света.
pub const LIGHT_POSITION: Vec3 = Vec3::new(12.0, 20.0, 10.0);

pub const GAME_OBJECTS_PATH: &str = "assets/game_objects.json";

pub const LEVEL_PATH: &str = "assets/levels/level01.txt";

pub const WINDOW_TITLE: &str = "Игра для крутых людей";
