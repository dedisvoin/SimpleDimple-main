use bevy::prelude::*;

use crate::data::{CELL_SIZE, MAP_CENTER, OBJECT_HEIGHT};
use crate::graphic_object::GraphicObject;

/// Игровой объект
/// Содержит логические координаты на карте и собственный графический объект,
/// которому делегирует вывод на экран (композиция).
#[derive(Component, Debug, Clone)]
pub struct GameObject {
    /// Логические координаты: x — столбец, y — строка карты.
    position: IVec2,

    /// Графический объект, представляющий игровой объект на экране.
    graphic_object: GraphicObject,
}

impl GameObject {
    pub fn new(graphic_object: GraphicObject, position: IVec2) -> Self {
        let mut object = Self {
            position: IVec2::ZERO,
            graphic_object,
        };
        object.set_position(position);
        object
    }

    /// Логические координаты объекта.
    pub fn position(&self) -> IVec2 {
        self.position
    }

    /// Задаёт логические координаты и пересчитывает графические.
    pub fn set_position(&mut self, position: IVec2) {
        self.position = position;
        self.graphic_object.position = logical_to_world(position);
    }

    /// Графический объект, которому делегируется вывод на экран.
    pub fn graphic_object(&self) -> &GraphicObject {
        &self.graphic_object
    }
}

/// Перевод логических координат ячейки в мировые координаты.
pub fn logical_to_world(position: IVec2) -> Vec3 {
    Vec3::new(
        (position.x - MAP_CENTER) as f32 * CELL_SIZE,
        OBJECT_HEIGHT,
        (position.y - MAP_CENTER) as f32 * CELL_SIZE,
    )
}

/// Создаёт игровой объект на сцене, делегируя вывод графическому объекту.
pub fn spawn_game_object(commands: &mut Commands, object: GameObject) -> Entity {
    let components = object.graphic_object().render_components();
    commands.spawn((object, components)).id()
}

/// Обновляет матрицу модели игровых объектов при изменении их координат.
pub fn sync_game_objects(mut query: Query<(&GameObject, &mut Transform), Changed<GameObject>>) {
    for (object, mut transform) in &mut query {
        transform.clone_from(&object.graphic_object().to_transform());
    }
}
