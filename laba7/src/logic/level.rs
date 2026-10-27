//! Игровой уровень: клетки карты, положение игрока и разбор описания уровня.

use std::{fs, path::Path};

use bevy::prelude::IVec2;

/// Содержимое клетки игрового уровня.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Свободная клетка.
    Empty,
    /// Лёгкий объект, который игрок может толкать.
    Light,
    /// Тяжёлый объект, который нельзя сдвинуть.
    Heavy,
    /// Граничный объект (стена).
    Border,
}

impl Cell {
    /// Возвращает `true`, если клетка свободна.
    pub fn is_free(self) -> bool {
        self == Self::Empty
    }
}

/// Направление перемещения игрока.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Влево по карте.
    Left,
    /// Вправо по карте.
    Right,
    /// Вверх по карте.
    Up,
    /// Вниз по карте.
    Down,
}

impl Direction {
    /// Смещение на одну клетку в этом направлении.
    pub fn offset(self) -> IVec2 {
        match self {
            Self::Left => IVec2::new(-1, 0),
            Self::Right => IVec2::new(1, 0),
            Self::Up => IVec2::new(0, -1),
            Self::Down => IVec2::new(0, 1),
        }
    }
}

/// Состояние уровня, независимое от представления сцены.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    width: i32,
    height: i32,
    /// Клетки построчно: клетка `(x, y)` хранится по индексу `y * width + x`.
    cells: Vec<Cell>,
    player: IVec2,
    moves: u32,
}

impl Level {
    /// Разбирает текст карты (`.`, `o`, `X`, `#` и ровно один `@`).
    ///
    /// Возвращает ошибку с номером строки и, если применимо, столбца.
    pub fn parse(description: &str) -> Result<Self, String> {
        let mut cells = Vec::new();
        let mut player = None;
        let mut width = None;
        let mut height = 0_i32;

        for (y, row) in description.lines().enumerate() {
            let row_width = row.chars().count();
            if row_width == 0 {
                return Err(format!("строка {}: пустая строка карты", y + 1));
            }
            if let Some(expected) = width {
                if row_width != expected {
                    return Err(format!(
                        "строка {}: длина {} вместо {expected}",
                        y + 1,
                        row_width
                    ));
                }
            } else {
                width = Some(row_width);
            }

            i32::try_from(row_width)
                .map_err(|_| format!("строка {}: ширина карты слишком велика", y + 1))?;
            let y_i32 = i32::try_from(y)
                .map_err(|_| format!("строка {}: высота карты слишком велика", y + 1))?;
            for (x, symbol) in row.chars().enumerate() {
                let cell = match symbol {
                    '.' => Cell::Empty,
                    'o' => Cell::Light,
                    'X' => Cell::Heavy,
                    '#' => Cell::Border,
                    '@' => {
                        if player.is_some() {
                            return Err(format!("строка {}: второй игрок", y + 1));
                        }
                        let x = i32::try_from(x)
                            .map_err(|_| format!("строка {}: столбец слишком велик", y + 1))?;
                        player = Some(IVec2::new(x, y_i32));
                        Cell::Empty
                    }
                    other => {
                        return Err(format!(
                            "строка {}, столбец {}: неизвестный символ '{other}'",
                            y + 1,
                            x + 1
                        ));
                    }
                };
                cells.push(cell);
            }
            height = y_i32
                .checked_add(1)
                .ok_or_else(|| format!("строка {}: высота карты слишком велика", y + 1))?;
        }

        let Some(width) = width else {
            return Err("строка 1: описание уровня пусто".to_owned());
        };
        let Some(player) = player else {
            return Err(format!("строка {}: игрок не указан", height + 1));
        };

        Ok(Self {
            width: i32::try_from(width)
                .map_err(|_| "строка 1: ширина карты слишком велика".to_owned())?,
            height,
            cells,
            player,
            moves: 0,
        })
    }

    /// Загружает уровень из файла и разбирает его методом [`Level::parse`].
    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let contents =
            fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
        Self::parse(&contents).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// Возвращает ширину карты в клетках.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Возвращает высоту карты в клетках.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Возвращает содержимое клетки; за границами карты находится стена.
    pub fn cell(&self, position: IVec2) -> Cell {
        self.index(position)
            .map_or(Cell::Border, |index| self.cells[index])
    }

    /// Возвращает логические координаты игрока.
    pub fn player(&self) -> IVec2 {
        self.player
    }

    /// Возвращает количество выполненных ходов.
    pub fn moves(&self) -> u32 {
        self.moves
    }

    /// Перебирает координаты и содержимое всех непустых клеток.
    pub fn objects(&self) -> impl Iterator<Item = (IVec2, Cell)> + '_ {
        let width = self.width as usize;
        self.cells
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, cell)| *cell != Cell::Empty)
            .map(move |(index, cell)| {
                (
                    IVec2::new((index % width) as i32, (index / width) as i32),
                    cell,
                )
            })
    }

    fn index(&self, position: IVec2) -> Option<usize> {
        if position.x < 0 || position.y < 0 || position.x >= self.width || position.y >= self.height
        {
            return None;
        }
        Some((position.y as usize) * (self.width as usize) + position.x as usize)
    }

    /// Перемещает клеточный объект. Вызывается правилами после проверки хода.
    pub(super) fn move_cell(&mut self, from: IVec2, to: IVec2) {
        if let (Some(from_index), Some(to_index)) = (self.index(from), self.index(to)) {
            self.cells[to_index] = self.cells[from_index];
            self.cells[from_index] = Cell::Empty;
        }
    }

    /// Перемещает игрока и учитывает выполненный ход.
    pub(super) fn move_player(&mut self, to: IVec2) {
        self.player = to;
        self.moves += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, Level};
    use bevy::prelude::IVec2;

    #[test]
    fn parse_reads_cells_and_player() {
        let level = Level::parse("#####\n#@oX#\n#####").unwrap();

        assert_eq!(level.width(), 5);
        assert_eq!(level.height(), 3);
        assert_eq!(level.player(), IVec2::new(1, 1));
        assert_eq!(level.cell(IVec2::new(1, 1)), Cell::Empty);
        assert_eq!(level.cell(IVec2::new(2, 1)), Cell::Light);
        assert_eq!(level.cell(IVec2::new(3, 1)), Cell::Heavy);
        assert_eq!(level.moves(), 0);
    }

    #[test]
    fn outside_of_map_is_border() {
        let level = Level::parse("###\n#@#\n###").unwrap();

        assert_eq!(level.cell(IVec2::new(-1, 1)), Cell::Border);
        assert_eq!(level.cell(IVec2::new(3, 1)), Cell::Border);
        assert_eq!(level.cell(IVec2::new(1, -1)), Cell::Border);
        assert_eq!(level.cell(IVec2::new(1, 3)), Cell::Border);
    }

    #[test]
    fn parse_rejects_bad_input() {
        let cases = [
            ("", "строка 1"),
            ("###", "строка 2"),
            ("####\n#@@#\n####", "строка 2"),
            ("###\n##", "строка 2"),
            ("###\n#@?\n###", "строка 2, столбец 3"),
        ];

        for (description, expected_error) in cases {
            let error = Level::parse(description).unwrap_err();
            assert!(error.contains(expected_error), "unexpected error: {error}");
        }
    }

    #[test]
    fn real_level_file_is_valid() {
        let level = Level::load("assets/levels/level01.txt").unwrap();

        assert_eq!(level.width(), 21);
        assert_eq!(level.height(), 21);
        assert_eq!(level.player(), IVec2::new(1, 19));
    }
}
