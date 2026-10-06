use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;

mod camera;
mod data;
mod display;
mod graphic_object;
mod simulation;
mod torus;

use camera::CameraRig;
use data::{initial_camera_position, scene_objects};
use display::{
    FPSText, FpsCounter, ObjectLabel, update_fps, update_object_labels,
};
use graphic_object::spawn_graphic_object;
use simulation::{apply_camera_transform, simulate_camera};
use torus::TorusPlugin;

/// Инициализация сцены: объекты, камера и освещение.
fn setup_scene(
    mut commands: Commands,           // команды для манипулирования сценой
    mut meshes: ResMut<Assets<Mesh>>, // ресурс хранящий меши
    mut materials: ResMut<Assets<StandardMaterial>>, // ресурс хранящий материалы
    camera_rig: Res<CameraRig>,
) {
    let graphic_objects = scene_objects(); // получаем список объектов сцены

    // Создаём все объекты из списка на сцене, нумеруя их от 1,
    // и запоминаем сущности торов для привязки UI-подписей.
    let mut torus_entities: Vec<Entity> = Vec::new();

    for (i, obj) in graphic_objects.into_iter().enumerate() {
        let entity = spawn_graphic_object(&mut commands, &mut meshes, &mut materials, obj);
        torus_entities.push(entity);

        // UI-подпись с номером объекта (Bevy UI, экранные координаты).
        commands.spawn((
            Text::new((i + 1).to_string()),
            TextFont::from_font_size(32.0),
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            ObjectLabel { torus: entity }, // привязка подписи к тору
        ));
    }

    // Создаём камеру в позиции из оригинальной лабораторной,
    // направленную в начало координат.
    commands.spawn((Camera3d::default(), camera_rig.transform()));

    // Добавляем направленный источник света.
    commands.spawn((
        DirectionalLight {
            illuminance: 3_000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 5.0, 7.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Добавляем фоновый (ambient) свет, чтобы объекты были видны и с теневой стороны.
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        ..default()
    });

    // Создаём текстовую метку FPS в левом верхнем углу окна.
    commands.spawn((
        Text::new("FPS: --"),
        TextFont::from_font_size(18.0),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
        FPSText,
    ));
}

/// Главная функция
fn main() {
    App::new()
        // Добавляем стандартные плагины Bevy.
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TorusPlugin)
        // Обновляем сцену каждый кадр без ограничений.
        .insert_resource(WinitSettings::continuous())
        .insert_resource(ClearColor(Color::srgb(0.02, 0.03, 0.09)))
        // Ресурсом добавляем палитру цветов.
        .insert_resource(CameraRig::new(initial_camera_position()))
        .insert_resource(FpsCounter::default())
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                simulate_camera,
                apply_camera_transform,
                update_object_labels,
                update_fps
            )
                .chain(),
        )
        .run();
}
