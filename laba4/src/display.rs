use std::time::Duration;

use crate::graphic_object::GraphicObject;
use bevy::prelude::*;

/// Счётчик кадров в секунду.
#[derive(Resource, Default)]
pub struct FpsCounter {
    frames: u32,
    elapsed: Duration,
    fps: f32,
}

/// Метка для [`Node`] содержащего текущий FPS.
#[derive(Component)]
pub struct FPSText;

/// Компонент UI-подписи объекта: хранит ссылку на тор, над которым подпись висит.
#[derive(Component)]
pub struct ObjectLabel {
    /// Сущность тора, к которому привязана подпись.
    pub torus: Entity,
}

/// Система обновления счётчика кадров в секунду.
pub fn update_fps(
    time: Res<Time>,
    mut counter: ResMut<FpsCounter>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    counter.frames += 1;
    counter.elapsed += time.delta();

    let elapsed_secs = counter.elapsed.as_secs_f32();

    if elapsed_secs >= 0.5 {
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
            window.title = format!("Lab04 [{fps} FPS]");
        }
    }
}

/// Система размещения UI-подписей.
///
/// Каждый кадр мировая позиция тора проецируется в координаты окна, и подпись
/// с его номером выводится чуть выше тора.
pub fn update_object_labels(
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut labels: Query<(&mut Node, &ObjectLabel)>,
    toruses: Query<&GlobalTransform, With<GraphicObject>>,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    for (mut node, label) in &mut labels {
        let Ok(torus) = toruses.get(label.torus) else {
            continue;
        };

        // Проекция центра тора в viewport-координаты (начало — левый верхний угол).
        let Some(viewport) = camera
            .world_to_viewport(camera_transform, torus.translation())
            .ok()
        else {
            continue;
        };

        // Подпись выводим чуть выше (по вертикали) от центра тора на экране.
        node.left = Val::Px(viewport.x - 12.0);
        node.top = Val::Px(viewport.y - 60.0);
    }
}
