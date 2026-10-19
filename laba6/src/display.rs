use std::time::Duration;

use bevy::prelude::*;

/// Счётчик кадров в секунду.
#[derive(Resource, Default)]
pub struct FpsCounter {
    frames: u32,
    elapsed: Duration,
    fps: f32,
}

/// Метка для [`Node`], содержащего текущий FPS.
#[derive(Component)]
pub struct FPSText;

/// Система обновления счётчика кадров в секунду.
pub fn update_fps(
    time: Res<Time>,
    mut counter: ResMut<FpsCounter>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    counter.frames += 1;
    counter.elapsed += time.delta();

    let elapsed_secs = counter.elapsed.as_secs_f32();

    if elapsed_secs >= 0.2 {
        counter.fps = (counter.frames as f32 / elapsed_secs).max(0.0);
        counter.frames = 0;
        counter.elapsed = Duration::ZERO;
    }

    if let Ok(mut text) = query.single_mut() {
        let fps = counter.fps.round().max(0.0) as u32;
        text.0 = format!("FPS: {fps}");
    }
}

/// Система обновления заголовка окна с текущим FPS.
pub fn update_window_title(
    mut windows: Query<&mut Window>,
    counter: Res<FpsCounter>,
    mut last_fps: Local<u32>,
) {
    let fps = counter.fps.round().max(0.0) as u32;

    if fps != *last_fps {
        *last_fps = fps;

        for mut window in &mut windows {
            window.title = format!("Lab06 [{fps} FPS]");
        }
    }
}
