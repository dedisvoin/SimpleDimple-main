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
                listen_keyboard,
                automatic_color_system,
                update_torus_color_system,
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
        info!("Создаём объект на позиции {:?}", obj.position);
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
    colors: Vec<Color>,
    current_index: usize,
    automatic: bool,
    timer: Timer,
}

impl ColorPalette {
    /// Конструктор для создания палитры цветов.
    fn new() -> Self {
        Self {
            colors: vec![
                Color::BLACK,
                Color::WHITE,
                Color::srgb(0.0, 0.0, 1.0),
                Color::srgb(1.0, 0.0, 0.0),
                Color::srgb(0.5, 0.0, 1.0),
            ],
            current_index: 0,
            automatic: false,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    /// Получаем текущий цвет.
    fn current_color(&self) -> Color {
        self.colors[self.current_index]
    }

    /// Функция для перехода к следующему цвету.
    fn next_color(&mut self) {
        self.current_index = (self.current_index + 1) % self.colors.len();
    }

    /// Функция для установки цвета по индексу.
    fn set_color(&mut self, index: usize) {
        self.current_index = index;
    }
}

/// Система обработки нажатий клавиатуры для управления палитрой.
fn listen_keyboard(keyboard: Res<ButtonInput<KeyCode>>, mut palette: ResMut<ColorPalette>) {
    // Пробел переключает следующий цвет.
    if keyboard.just_pressed(KeyCode::Space) {
        palette.next_color();
    }

    // 1-5 переключают цвет по индексу.
    if keyboard.just_pressed(KeyCode::Digit1) {
        palette.set_color(0);
    }

    if keyboard.just_pressed(KeyCode::Digit2) {
        palette.set_color(1);
    }

    if keyboard.just_pressed(KeyCode::Digit3) {
        palette.set_color(2);
    }

    if keyboard.just_pressed(KeyCode::Digit4) {
        palette.set_color(3);
    }

    if keyboard.just_pressed(KeyCode::Digit5) {
        palette.set_color(4);
    }

    // A переключает автоматическую систему смены цвета.
    if keyboard.just_pressed(KeyCode::KeyA) {
        palette.automatic = !palette.automatic;
    }
}

/// Система автоматической смены цвета при включённом таймере.
fn automatic_color_system(time: Res<Time>, mut palette: ResMut<ColorPalette>) {
    // Цвет меняется, если автопереключение включено и таймер завершился в этом кадре.
    if palette.automatic && palette.timer.tick(time.delta()).just_finished() {
        palette.next_color();
    }
}

/// Обновляет цвет всех графических объектов на основе текущего цвета палитры.
fn update_torus_color_system(
    palette: Res<ColorPalette>,
    mut materials: ResMut<Assets<StandardMaterial>>, // ресурс всех материалов
    query: Query<&MeshMaterial3d<StandardMaterial>, With<GraphicObject>>, // материалы торов
) {
    for material_handle in &query {
        // Пытаемся получить материал по его дескриптору (.0).
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = palette.current_color(); // устанавливаем цвет
        }
    }
}
