mod graphic_object;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;
use graphic_object::{GraphicObject, spawn_graphic_object};

/// Точка входа в программу.
fn main() {
    App::new()
        // Добавляем стандартные плагины Bevy.
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Отключаем vsync, чтобы не было ограничения частоты кадров.
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // Обновляем сцену каждый кадр без ограничений.
        .insert_resource(WinitSettings::continuous())
        // Устанавливаем чёрный фон сцены, как в оригинальном Lab 3.
        .insert_resource(ClearColor(Color::BLACK))
        // Добавляем функцию setup_scene как инициализирующую.
        .add_systems(Startup, setup_scene)
        // Обновляем счётчик FPS каждый кадр.
        .add_systems(Update, (update_fps_text, torus_orbit_system))
        .run();
}

/// Инициализация сцены: объекты, камера и освещение.
fn setup_scene(
    mut commands: Commands,           // команды для манипулирования сценой
    mut meshes: ResMut<Assets<Mesh>>, // ресурс хранящий меши
    mut materials: ResMut<Assets<StandardMaterial>>, // ресурс хранящий материалы
) {
    // Список графических объектов (аналог std::vector в C++).
    // Каждый объект стоит на одной из осей OX/OZ на расстоянии 4 от центра,
    // а его «носик» (локальная +X) повернут к началу координат.
    let graphic_objects: Vec<GraphicObject> = vec![
        // Красный — справа (+X), угол 180°
        GraphicObject::new(Vec3::new(4.0, 0.0, 0.0), 180.0, Vec3::new(1.0, 0.0, 0.0)),
        // Синий — слева (−X), угол 0°
        GraphicObject::new(Vec3::new(-4.0, 0.0, 0.0), 0.0, Vec3::new(0.0, 0.0, 1.0)),
        // Зелёный — сзади (−Z), угол 90°
        GraphicObject::new(Vec3::new(0.0, 0.0, -4.0), 90.0, Vec3::new(0.0, 1.0, 0.0)),
        // Белый — спереди (+Z), угол 270°
        GraphicObject::new(Vec3::new(0.0, 0.0, 4.0), 270.0, Vec3::new(1.0, 1.0, 1.0)),
    ];

    // Создаём все объекты из списка на сцене, нумеруя их от 1.
    for (i, obj) in graphic_objects.into_iter().enumerate() {
        info!("Создаём объект на позиции {:?}", obj.position);
        spawn_graphic_object(&mut commands, &mut meshes, &mut materials, obj, i + 1);
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

/// Скорость вращения объектов вокруг центра сцены (радианы в секунду).
const TORUS_ORBIT_SPEED: f32 = 0.5;

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
