//! Представление уровня и игровые сущности сцены.

mod factory;
mod game_object;
mod graphic_object;
mod light;
mod mesh;

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::{CELL_SIZE, GAME_OBJECTS_PATH, LIGHT_POSITION, OBJECT_HEIGHT},
    game::CurrentLevel,
    schedule::GameSet,
};

pub use factory::{GameObjectFactory, GameObjectType};
use game_object::sync_game_objects;
pub use game_object::{GameObject, spawn_game_object};
pub use graphic_object::GraphicObject;
pub use light::{SceneLight, apply_light, spawn_scene_light};

/// Маркер сущности игрока.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Player;

/// Связь клеток уровня с сущностями, которые их изображают.
#[derive(Resource, Default)]
pub struct CellEntities(HashMap<IVec2, Entity>);

impl CellEntities {
    /// Добавляет или заменяет сущность для клетки.
    pub fn insert(&mut self, cell: IVec2, entity: Entity) -> Option<Entity> {
        self.0.insert(cell, entity)
    }

    /// Возвращает сущность, отображающую клетку.
    #[allow(dead_code)]
    pub fn get(&self, cell: IVec2) -> Option<Entity> {
        self.0.get(&cell).copied()
    }

    /// Переносит связь сущности с одной клетки на другую.
    pub fn relocate(&mut self, from: IVec2, to: IVec2) -> Option<Entity> {
        let entity = self.0.remove(&from)?;
        self.0.insert(to, entity);
        Some(entity)
    }
}

/// Геометрическая раскладка карты в мировых координатах.
#[derive(Debug, Clone, Copy)]
pub(super) struct BoardLayout {
    center: Vec2,
}

impl BoardLayout {
    /// Вычисляет центр карты по её ширине и высоте в клетках.
    pub(super) fn new(width: i32, height: i32) -> Self {
        Self {
            center: Vec2::new((width as f32 - 1.0) / 2.0, (height as f32 - 1.0) / 2.0),
        }
    }

    /// Переводит координаты клетки в мировые координаты сцены.
    pub(super) fn to_world(self, position: Vec2) -> Vec3 {
        let offset = (position - self.center) * CELL_SIZE;
        Vec3::new(offset.x, OBJECT_HEIGHT, offset.y)
    }
}

/// Плагин создания сцены, анимации объектов и обновления освещения.
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CellEntities>()
            .insert_resource(SceneLight::new(LIGHT_POSITION))
            .add_systems(Startup, (setup_scene, setup_scene_light))
            .add_systems(Update, animate_game_objects.in_set(GameSet::Animation))
            .add_systems(
                Update,
                (sync_game_objects, apply_light).in_set(GameSet::Render),
            );
    }
}

/// Создаёт сущности объектов уровня и игрока с помощью фабрики.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level: Res<CurrentLevel>,
    mut cell_entities: ResMut<CellEntities>,
) {
    let layout = BoardLayout::new(level.0.width(), level.0.height());
    let factory = GameObjectFactory::init(&mut meshes, &mut materials, layout, GAME_OBJECTS_PATH)
        .unwrap_or_else(|error| panic!("не удалось создать фабрику объектов: {error}"));

    for (position, cell) in level.0.objects() {
        let Some(kind) = GameObjectType::from_cell(cell) else {
            continue;
        };

        let object = factory
            .create(kind, position)
            .unwrap_or_else(|error| panic!("не удалось создать объект: {error}"));
        let entity = spawn_game_object(&mut commands, object);
        cell_entities.insert(position, entity);
    }

    let player_object = factory
        .create(GameObjectType::PlayerObject, level.0.player())
        .unwrap_or_else(|error| panic!("не удалось создать игрока: {error}"));
    let player_entity = spawn_game_object(&mut commands, player_object);
    commands.entity(player_entity).insert(Player);

    // Фабрика хранит общие handles мешей и материалов объектов.
    commands.insert_resource(factory);
}

/// Создаёт сущность точечного источника света.
fn setup_scene_light(mut commands: Commands, light: Res<SceneLight>) {
    spawn_scene_light(&mut commands, &light);
}

/// Продвигает анимацию всех игровых объектов на время прошедшего кадра.
fn animate_game_objects(time: Res<Time>, mut objects: Query<&mut GameObject>) {
    let seconds = time.delta_secs();
    for mut object in &mut objects {
        object.simulate(seconds);
    }
}
