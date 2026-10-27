//! Правила проверки и выполнения хода игрока.

use bevy::prelude::IVec2;

use super::{Cell, Direction, Level};

/// Результат проверки допустимого хода.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    /// Направление хода.
    pub direction: Direction,
    /// Клетка игрока до хода.
    pub from: IVec2,
    /// Клетка игрока после хода.
    pub to: IVec2,
    /// Клетка лёгкого объекта до толчка, если игрок его толкает.
    pub pushed: Option<IVec2>,
}

/// Проверяет ход по правилам игры, не изменяя уровень.
///
/// Возвращает описание хода, если соседняя клетка свободна либо в ней стоит
/// лёгкий объект с пустой клеткой за ним. Иначе возвращает `None`.
pub fn plan_move(level: &Level, direction: Direction) -> Option<Move> {
    let from = level.player();
    let offset = direction.offset();
    let to = from + offset;
    let behind = to + offset;

    let pushed = match level.cell(to) {
        Cell::Empty => None,
        Cell::Light if level.cell(behind).is_free() => Some(to),
        Cell::Light | Cell::Heavy | Cell::Border => return None,
    };

    Some(Move {
        direction,
        from,
        to,
        pushed,
    })
}

/// Применяет ход, ранее полученный от [`plan_move`].
///
/// Если ход толкает лёгкий объект, сначала переносит его на соседнюю клетку,
/// затем перемещает игрока и увеличивает счётчик ходов.
pub fn apply_move(level: &mut Level, step: &Move) {
    if let Some(object) = step.pushed {
        let object_destination = object + step.direction.offset();
        level.move_cell(object, object_destination);
    }

    level.move_player(step.to);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::IVec2;

    use super::{apply_move, plan_move};
    use crate::logic::{Cell, Direction, Level};

    fn level(description: &str) -> Level {
        Level::parse(description).unwrap()
    }

    #[test]
    fn walk_into_empty_cell() {
        let mut level = level("#####\n#@..#\n#####");
        let step = plan_move(&level, Direction::Right).unwrap();

        assert_eq!(step.from, IVec2::new(1, 1));
        assert_eq!(step.to, IVec2::new(2, 1));
        assert_eq!(step.pushed, None);
        apply_move(&mut level, &step);

        assert_eq!(level.player(), IVec2::new(2, 1));
        assert_eq!(level.moves(), 1);
    }

    #[test]
    fn walls_and_heavy_objects_block() {
        assert_eq!(
            plan_move(&level("#####\n#@X.#\n#####"), Direction::Right),
            None
        );
        assert_eq!(plan_move(&level("###\n#@#\n###"), Direction::Right), None);
    }

    #[test]
    fn push_light_object_into_free_cell() {
        let mut level = level("######\n#@o..#\n######");
        let step = plan_move(&level, Direction::Right).unwrap();
        assert_eq!(step.pushed, Some(IVec2::new(2, 1)));

        apply_move(&mut level, &step);

        assert_eq!(level.player(), IVec2::new(2, 1));
        assert_eq!(level.cell(IVec2::new(2, 1)), Cell::Empty);
        assert_eq!(level.cell(IVec2::new(3, 1)), Cell::Light);
        assert_eq!(level.moves(), 1);
    }

    #[test]
    fn light_object_cannot_be_pushed_into_obstacle() {
        assert_eq!(
            plan_move(&level("#####\n#@oX#\n#####"), Direction::Right),
            None
        );
        assert_eq!(
            plan_move(&level("#####\n#@oo#\n#####"), Direction::Right),
            None
        );
        assert_eq!(
            plan_move(&level("####\n#@o#\n####"), Direction::Right),
            None
        );
    }

    #[test]
    fn rejected_move_changes_nothing() {
        let level = level("#####\n#@X.#\n#####");
        let original = level.clone();

        assert_eq!(plan_move(&level, Direction::Right), None);
        assert_eq!(level, original);
    }
}
