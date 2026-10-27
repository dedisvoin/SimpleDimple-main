use bevy::prelude::*;

use super::BoardLayout;
use super::graphic_object::GraphicObject;
use crate::logic::Direction;

/// Текущее перемещение объекта.
#[derive(Debug, Clone, Copy)]
struct Motion {
    /// Направление перемещения.
    direction: Direction,
    /// Прогресс: от 0.0 (начало) до 1.0 (объект в целевой клетке).
    progress: f32,
    /// Скорость, клеток в секунду.
    speed: f32,
}

/// Игровой объект: клетка, текущее перемещение и графический объект
/// для вывода на экран (композиция, как в ЛР №6).
#[derive(Component, Debug, Clone)]
pub struct GameObject {
    /// Логические координаты. Во время перемещения — целевая клетка.
    position: IVec2,
    /// Перемещение или `None`, если объект стоит на месте.
    motion: Option<Motion>,
    layout: BoardLayout,
    graphic_object: GraphicObject,
}

impl GameObject {
    pub fn new(graphic_object: GraphicObject, position: IVec2, layout: BoardLayout) -> Self {
        let mut object = Self {
            position: IVec2::ZERO,
            motion: None,
            layout,
            graphic_object,
        };
        object.set_position(position);
        object
    }

    /// Логические координаты объекта.
    #[allow(dead_code)]
    pub fn position(&self) -> IVec2 {
        self.position
    }

    /// Задаёт логические координаты и пересчитывает графические.
    pub fn set_position(&mut self, position: IVec2) {
        self.position = position;
        self.motion = None;
        self.update_graphic_position();
    }

    /// Графический объект, которому делегируется вывод на экран.
    pub fn graphic_object(&self) -> &GraphicObject {
        &self.graphic_object
    }

    fn update_graphic_position(&mut self) {
        let lag = self.motion.map_or(Vec2::ZERO, |motion| {
            motion.direction.offset().as_vec2() * (1.0 - motion.progress)
        });
        self.graphic_object.position = self.layout.to_world(self.position.as_vec2() - lag);
    }

    pub fn start_move(&mut self, direction: Direction, speed: f32) {
        if self.is_moving() {
            return;
        }
        self.position += direction.offset();
        self.motion = Some(Motion {
            direction,
            progress: 0.0,
            speed,
        });
        self.update_graphic_position();
    }

    pub fn simulate(&mut self, seconds: f32) {
        let Some(motion) = &mut self.motion else {
            return;
        };
        motion.progress = (motion.progress + motion.speed * seconds).min(1.0);
        if motion.progress >= 1.0 {
            self.motion = None;
        }
        self.update_graphic_position();
    }

    pub fn is_moving(&self) -> bool {
        self.motion.is_some()
    }
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
