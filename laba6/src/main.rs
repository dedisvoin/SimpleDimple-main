use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;

mod camera;
mod data;
mod display;
mod game_object;
mod graphic_object;
mod light;
mod mesh;
mod phong_material;
mod simulation;

use camera::CameraRig;
use data::{
    MAP_SIZE, MATERIAL_FILES, MESH_FILES, MapObjects, PASSABILITY_MAP, SceneAssets,
    initial_camera_position, initial_light_position, object_type,
};
use display::{FPSText, FpsCounter, update_fps, update_window_title};
use game_object::{GameObject, spawn_game_object, sync_game_objects};
use graphic_object::{GraphicObject, spawn_graphic_object};
use light::{SceneLight, apply_light, spawn_scene_light};
use mesh::ObjMesh;
use phong_material::PhongMaterial;
use simulation::{apply_camera_transform, simulate_camera};

/// Инициализация сцены: загрузка мешей и материалов, создание основания сцены
/// и игровых объектов по карте проходимости.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_rig: Res<CameraRig>,
    light: Res<SceneLight>,
) {
    // Загружаем меши из obj-файлов и материалы из текстовых файлов.
    let mut assets = SceneAssets::default();

    for path in MESH_FILES {
        let obj = ObjMesh::load(path).unwrap_or_else(|error| {
            eprintln!("Ошибка загрузки меша: {error}");
            std::process::exit(1);
        });
        assets.meshes.push(meshes.add(obj.to_bevy_mesh()));
    }

    for path in MATERIAL_FILES {
        let phong = PhongMaterial::load(path).unwrap_or_else(|error| {
            eprintln!("Ошибка загрузки материала: {error}");
            PhongMaterial::default()
        });
        println!("Загружен материал {path}: {phong:?}");
        assets
            .materials
            .push(materials.add(phong.to_standard_material()));
    }

    // Основание сцены — горизонтальная плоскость во всю карту.
    let plane = GraphicObject::new(
        Vec3::ZERO,
        0.0,
        assets.meshes[0].clone(),
        assets.materials[0].clone(),
    );
    spawn_graphic_object(&mut commands, plane);

    // Игровые объекты по карте проходимости.
    let mut map_objects = MapObjects::default();
    let mut object_count = 0;

    for (y, row) in PASSABILITY_MAP.iter().enumerate() {
        for (x, &kind) in row.iter().enumerate() {
            let Some((mesh_index, material_index)) = object_type(kind) else {
                continue;
            };

            let position = IVec2::new(x as i32, y as i32);

            // В одной ячейке не может находиться два объекта.
            if map_objects.get(position).is_some() {
                eprintln!("Ячейка {position} уже занята");
                continue;
            }

            let graphic_object = GraphicObject::new(
                Vec3::ZERO,
                0.0,
                assets.meshes[mesh_index].clone(),
                assets.materials[material_index].clone(),
            );
            let game_object = GameObject::new(graphic_object, position);
            let cell = game_object.position();
            let entity = spawn_game_object(&mut commands, game_object);
            map_objects.set(cell, entity);
            object_count += 1;
        }
    }

    commands.insert_resource(assets);
    commands.insert_resource(map_objects);

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
        "[setup_scene] Сцена создана: {object_count} объектов на карте {MAP_SIZE}x{MAP_SIZE}, \
         источник света в {:?}.",
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
                sync_game_objects,
                update_fps,
                update_window_title,
            )
                .chain(),
        )
        .run();
}
