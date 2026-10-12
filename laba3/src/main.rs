mod graphic_object;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;
use graphic_object::{GraphicObject, spawn_graphic_object};


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
        // Обновляем сцену каждый кадр без ограничений.
        .insert_resource(WinitSettings::continuous())
        .insert_resource(ClearColor(Color::srgb(0.02, 0.03, 0.09)))
        // Ресурсом добавляем палитру цветов.
        .insert_resource(ColorPalette::new())

        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                update_fps_text,
                torus_orbit_system,
                update_object_labels,
                simulation,
            ),
        )
        .run();
}

/// Инициализация сцены: объекты, камера и освещение.
fn setup_scene(
    mut commands: Commands,           // команды для манипулирования сценой
    mut meshes: ResMut<Assets<Mesh>>, // ресурс хранящий меши
    mut materials: ResMut<Assets<StandardMaterial>>, // ресурс хранящий материалы
) {
    let graphic_objects: Vec<GraphicObject> = vec![
        GraphicObject::new(Vec3::new(4.0, 0.0, 0.0), 180.0, Vec3::new(1.0, 0.0, 0.0)),
        GraphicObject::new(Vec3::new(-4.0, 0.0, 0.0), 0.0, Vec3::new(0.0, 0.0, 1.0)),
        GraphicObject::new(Vec3::new(0.0, 0.0, -4.0), 90.0, Vec3::new(0.0, 1.0, 0.0)),
        GraphicObject::new(Vec3::new(0.0, 0.0, 4.0), 270.0, Vec3::new(1.0, 1.0, 1.0)),
    ];

    // Создаём все объекты из списка на сцене, нумеруя их от 1,
    // и запоминаем сущности торов для привязки UI-подписей.
    let mut torus_entities: Vec<Entity> = Vec::new();

    for (i, obj) in graphic_objects.into_iter().enumerate() {
        let entity = spawn_graphic_object(
            &mut commands,
            &mut meshes,
            &mut materials,
            obj,
            i % PALETTE.len(),
            &PALETTE,
        );
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
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 15.0, 17.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Добавляем направленный источник света.
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        FpsText, // маркер для поиска метки FPS
    ));
}

/// Маркер-компонент для текстовой метки FPS.
#[derive(Component)]
struct FpsText;

/// Система обновления счётчика кадров в секунду.
fn update_fps_text(time: Res<Time>, mut query: Query<&mut Text, With<FpsText>>) {
    if let Ok(mut text) = query.single_mut() {
        let fps = 1.0 / time.delta_secs();
        text.0 = format!("FPS: {fps:.0}");
    }
}

/// Компонент UI-подписи объекта: хранит ссылку на тор, над которым подпись висит.
#[derive(Component)]
struct ObjectLabel {
    /// Сущность тора, к которому привязана подпись.
    torus: Entity,
}
const TORUS_ORBIT_SPEED: f32 = 0.5;

const PALETTE: [Vec3; 5] = [
    Vec3::new(0.0, 0.0, 0.0), // чёрный
    Vec3::new(1.0, 1.0, 1.0), // белый
    Vec3::new(0.0, 0.0, 1.0), // синий
    Vec3::new(1.0, 0.0, 0.0), // красный
    Vec3::new(0.5, 0.0, 1.0), // фиолетовый
];

/// Система вращения всех графических объектов вокруг центра координат.
///
/// Каждый кадр матрица модели объекта поворачивается вокруг начала координат:
/// торы движутся по окружности, а их «носики» остаются направленными в центр.
fn torus_orbit_system(time: Res<Time>, mut query: Query<&mut Transform, With<GraphicObject>>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(TORUS_ORBIT_SPEED * time.delta_secs()),
        );
    }
}

/// Система размещения UI-подписей.
///
/// Каждый кадр мировая позиция тора проецируется в координаты окна, и подпись
/// с его номером выводится чуть выше тора. Задние/вышедшие за кадр торы не
/// получают подпись.
fn update_object_labels(
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

/// Палитра цветов для перекрашивания графических объектов.
#[derive(Resource)]
struct ColorPalette {
    colors: Vec<Vec3>,
    timer: Timer,
}

impl ColorPalette {
    /// Конструктор для создания палитры цветов.
    fn new() -> Self {
        Self {
            colors: PALETTE.to_vec(),
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

/// Система симуляции: раз в секунду сдвигает цвет каждого тора
/// на следующий в палитре.
fn simulation(
    time: Res<Time>,
    mut palette: ResMut<ColorPalette>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut GraphicObject,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    // Тикаем общий таймер. Если секунда ещё не прошла — ничего не делаем.
    if !palette.timer.tick(time.delta()).just_finished() {
        return;
    }

    let palette_len = palette.colors.len();

    for (mut object, material_handle) in &mut query {
        // Сдвигаем индекс цвета у объекта.
        object.advance_color(palette_len);
        object.color = palette.colors[object.color_index];

        // Обновляем материал этого конкретного тора.
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = object.to_color();
        }
    }
}