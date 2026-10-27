//! Модель игры и правила перемещения, не зависящие от Bevy ECS.

mod level;
mod rules;

pub use level::{Cell, Direction, Level};
pub use rules::{apply_move, plan_move};
