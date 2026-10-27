//! Управление игрой: загрузка уровня, ввод команд и применение правил.

use bevy::prelude::*;

use crate::{
    config::{LEVEL_PATH, PLAYER_SPEED, PUSH_SPEED},
    logic::{Direction, Level, apply_move, plan_move},
    scene::{CellEntities, GameObject, Player},
    schedule::GameSet,
};

/// Текущий уровень — состояние игры.
///
/// Обёртка позволяет хранить уровень как ECS-ресурс, не связывая модуль
/// `logic` с Bevy ECS.
#[derive(Resource)]
pub struct CurrentLevel(pub Level);

/// Команда игрока сделать ход в указанном направлении.
#[derive(Message, Debug, Clone, Copy)]
pub struct MoveRequest(pub Direction);

/// Плагин управления игрой и применения игровых правил.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let level = Level::load(LEVEL_PATH)
            .unwrap_or_else(|error| panic!("не удалось загрузить уровень: {error}"));

        app.insert_resource(CurrentLevel(level))
            .add_message::<MoveRequest>()
            .add_systems(Update, read_player_input.in_set(GameSet::Input))
            .add_systems(Update, apply_game_rules.in_set(GameSet::Logic));
    }
}

/// Читает нажатия WASD и отправляет запрос на ход.
fn read_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<MoveRequest>,
) {
    if let Some(direction) = pressed_direction(&keyboard) {
        requests.write(MoveRequest(direction));
    }
}

/// Возвращает направление для первой нажатой клавиши из WASD.
fn pressed_direction(keyboard: &ButtonInput<KeyCode>) -> Option<Direction> {
    if keyboard.pressed(KeyCode::KeyW) {
        Some(Direction::Up)
    } else if keyboard.pressed(KeyCode::KeyA) {
        Some(Direction::Left)
    } else if keyboard.pressed(KeyCode::KeyS) {
        Some(Direction::Down)
    } else if keyboard.pressed(KeyCode::KeyD) {
        Some(Direction::Right)
    } else {
        None
    }
}

/// Проверяет последнюю команду кадра, меняет модель и запускает анимацию.
fn apply_game_rules(
    mut requests: MessageReader<MoveRequest>,
    mut level: ResMut<CurrentLevel>,
    mut cell_entities: ResMut<CellEntities>,
    mut player: Single<&mut GameObject, With<Player>>,
    mut objects: Query<&mut GameObject, Without<Player>>,
) {
    // Все сообщения кадра нужно прочитать, даже если выполняется последнее.
    let Some(MoveRequest(direction)) = requests.read().last().copied() else {
        return;
    };

    // Пока игрок не завершил текущее перемещение, новые команды игнорируются.
    if player.is_moving() {
        return;
    }

    let Some(step) = plan_move(&level.0, direction) else {
        return;
    };
    apply_move(&mut level.0, &step);

    let player_speed = match step.pushed {
        Some(from) => {
            let to = from + direction.offset();
            if let Some(entity) = cell_entities.relocate(from, to)
                && let Ok(mut object) = objects.get_mut(entity)
            {
                object.start_move(direction, PUSH_SPEED);
            }
            PUSH_SPEED
        }
        None => PLAYER_SPEED,
    };

    player.start_move(direction, player_speed);
}
