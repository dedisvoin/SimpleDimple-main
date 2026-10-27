//! Отображение FPS и количества выполненных ходов в заголовке окна.

use std::time::Duration;

use bevy::prelude::*;

use crate::{game::CurrentLevel, schedule::GameSet};

/// Счётчик кадров в секунду.
#[derive(Resource, Default)]
pub struct FpsCounter {
    frames: u32,
    elapsed: Duration,
    fps: u32,
}

/// Плагин подсчёта FPS и обновления заголовка окна.
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FpsCounter>()
            .add_systems(Update, update_fps.in_set(GameSet::Render))
            .add_systems(Update, update_window_title.in_set(GameSet::Render));
    }
}

/// Подсчитывает средний FPS за интервалы примерно по 0,2 секунды.
fn update_fps(time: Res<Time>, mut counter: ResMut<FpsCounter>) {
    counter.frames += 1;
    counter.elapsed += time.delta();

    let elapsed_secs = counter.elapsed.as_secs_f32();
    if elapsed_secs >= 0.2 {
        counter.fps = (counter.frames as f32 / elapsed_secs).max(0.0).round() as u32;
        counter.frames = 0;
        counter.elapsed = Duration::ZERO;
    }
}

/// Показывает FPS и число успешных ходов в заголовке каждого окна.
fn update_window_title(
    mut windows: Query<&mut Window>,
    fps: Res<FpsCounter>,
    level: Res<CurrentLevel>,
    mut last_title: Local<String>,
) {
    let title = format!("AOKG [{} FPS] [Ходы: {}]", fps.fps, level.0.moves());
    if title == *last_title {
        return;
    }

    *last_title = title.clone();
    for mut window in &mut windows {
        window.title.clone_from(&title);
    }
}
