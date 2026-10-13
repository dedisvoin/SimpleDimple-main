use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;

mod camera;
mod data;
mod display;
mod graphic_object;
mod light;
mod phong_material;
mod simulation;

use camera::CameraRig;
use data::{MATERIAL_FILES, initial_camera_position, initial_light_position, scene_layout};
use display::{FPSText, FpsCounter, ObjectLabel, update_fps, update_object_labels};
use graphic_object::{GraphicObject, spawn_graphic_object};
use light::{SceneLight, apply_light, spawn_scene_light};
use phong_material::PhongMaterial;
use simulation::{apply_camera_transform, simulate_camera};

/// Инициализация сцены: объекты, материалы, источник света и камера.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_rig: Res<CameraRig>,
    light: Res<SceneLight>,
) {

    let torus = meshes.add(Torus::new(0.4, 1.0));

    // Загружаем материалы из файлов.
    let material_handles: Vec<Handle<StandardMaterial>> = MATERIAL_FILES
        .iter()
        .map(|path| {
            let phong = PhongMaterial::load(path).unwrap_or_else(|error| {
                eprintln!("Ошибка загрузки материала: {error}");
                PhongMaterial::default()
            });
            println!("Загружен материал {path}: {phong:?}");
            materials.add(phong.to_standard_material())
        })
        .collect();

    // Создаём графические объекты.
    let mut torus_entities: Vec<Entity> = Vec::new();

    for (i, layout) in scene_layout().into_iter().enumerate() {
        let material = material_handles[layout.material_index].clone();
        let object = GraphicObject::new(layout.position, layout.angle_deg, material);
        let entity = spawn_graphic_object(&mut commands, torus.clone(), object);
        torus_entities.push(entity);

        // UI-подпись с номером объекта.
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
            ObjectLabel { torus: entity },
        ));
    }

    // Точечный источник света.
    spawn_scene_light(&mut commands, &light);

    // Камера.
    commands.spawn((Camera3d::default(), camera_rig.transform()));

    // Текстовая метка FPS.
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

    println!(
        "[setup_scene] Сцена создана: {} объекта, источник света в {:?}.",
        torus_entities.len(),
        light.position()
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::continuous())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(CameraRig::new(initial_camera_position()))
        .insert_resource(SceneLight::new(initial_light_position()))
        .insert_resource(FpsCounter::default())
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                simulate_camera,
                apply_camera_transform,
                apply_light,
                update_object_labels,
                update_fps,
            )
                .chain(),
        )
        .run();
}
