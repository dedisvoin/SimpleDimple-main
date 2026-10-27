//! Порядок выполнения игровых систем в течение кадра.

use bevy::{
    ecs::schedule::IntoScheduleConfigs,
    prelude::{App, Plugin, SystemSet, Update},
};

/// Этап обработки кадра.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSet {
    /// Чтение команд игрока и управление камерой.
    Input,
    /// Проверка и применение игровых ходов.
    Logic,
    /// Плавное перемещение объектов между клетками.
    Animation,
    /// Обновление компонентов отображения: моделей, камеры, света и HUD.
    Render,
}

/// Настраивает порядок этапов обновления игры.
pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Logic,
                GameSet::Animation,
                GameSet::Render,
            )
                .chain(),
        );
    }
}
