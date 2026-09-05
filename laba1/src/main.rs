//! Лабораторная работа №1 (Bevy Engine 0.19)
//! Инициализация Bevy, использование ECS и вывод 3D-объекта (тора)
//! с возможностью циклического изменения цвета.

use bevy::{
    prelude::*,
    time::TimerMode,
};

// ═══════════════════════════════════════════════════════════════════════════════
// РЕСУРСЫ (Resources) — глобально уникальные данные
// ═══════════════════════════════════════════════════════════════════════════════

/// Ресурс, хранящий массив цветов для циклического переключения.
/// Аналог глобального массива в оригинальной программе на OpenGL/freeglut.
#[derive(Resource)]
struct ColorPalette {
    /// Массив цветов в формате (R, G, B, A).
    /// Порядок: чёрный, белый, синий, красный, фиолетовый.
    colors: Vec<Color>,
    /// Текущий индекс в массиве цветов.
    current_index: usize,
}

impl ColorPalette {
    fn new() -> Self {
        Self {
            colors: vec![
                Color::srgb(0.0, 0.0, 0.0),         // чёрный
                Color::srgb(1.0, 1.0, 1.0),         // белый
                Color::srgb(0.0, 0.0, 1.0),         // синий
                Color::srgb(1.0, 0.0, 0.0),         // красный
                Color::srgb(0.5, 0.0, 0.5),         // фиолетовый
            ],
            current_index: 0,
        }
    }

    /// Переключить на следующий цвет в массиве (циклически).
    fn next_color(&mut self) -> Color {
        self.current_index = (self.current_index + 1) % self.colors.len();
        self.colors[self.current_index]
    }
}

/// Ресурс-таймер для автоматической смены цвета.
/// Аналог glutTimerFunc(20, simulation, 0) в оригинальной программе.
#[derive(Resource)]
struct AutoColorTimer(Timer);

// ═══════════════════════════════════════════════════════════════════════════════
// КОМПОНЕНТЫ (Components) — данные, привязанные к сущностям
// ═══════════════════════════════════════════════════════════════════════════════

/// Компонент, помечающий торовую сетку как «главный объект лабораторной работы».
/// В Bevy ECS компоненты — это обычные Rust-структуры с производным трейтом Component.
#[derive(Component)]
struct LabObject;

// ═══════════════════════════════════════════════════════════════════════════════
// СИСТЕМЫ (Systems) — логика, обрабатывающая данные
// ═══════════════════════════════════════════════════════════════════════════════

/// Стартовая система (Startup): создаёт 3D-тор — аналог вызова glutWireTeapot(1.0).
/// Запускается ровно один раз при старте приложения.
fn setup_torus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Создаём меш тора (inner_radius = 0.4, outer_radius = 1.0)
    // В Bevy 0.19 Torus::new принимает только два параметра: внутренний и внешний радиус.
    // Аналог glutWireTeapot(1.0) из OpenGL/freeglut.
    let torus_mesh = meshes.add(Torus::new(0.4, 1.0));

    // Начальный цвет — красный (1.0, 0.0, 0.0), как в оригинальной программе
    let torus_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..default()
    });

    // Создаём сущность (Entity) и привязываем к ней компоненты:
    // - Mesh3d — 3D-меш для рендеринга
    // - MeshMaterial3d — материал (цвет, текстура и т.д.)
    // - LabObject — наш пользовательский маркер
    // - Transform — положение, поворот, масштаб в 3D-пространстве
    //     Аналог вызовов glTranslatef / glRotatef в OpenGL
    commands.spawn((
        Mesh3d(torus_mesh),
        MeshMaterial3d(torus_material),
        LabObject,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Добавляем направленный свет (аналог настройки освещения в OpenGL)
    // В Bevy 0.19 DirectionalLight использует поле illuminance.
    commands.spawn((
        DirectionalLight {
            illuminance: 1_000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 5.0, 7.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Окружающий свет — в Bevy 0.19 это РЕСУРС GlobalAmbientLight,
    // а не компонент AmbientLight (как было в старых версиях).
    // Аналог: глобальное окружное освещение OpenGL (GL_LIGHT_MODEL_AMBIENT).
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.4, 0.4, 0.4),
        brightness: 1.0,
        ..default()
    });

    // Камера: аналог gluLookAt(5, 5, 7.5, 0, 0, 0, 0, 1, 0) + gluPerspective
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 7.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    println!("[setup_torus] Тор создан. Нажмите пробел для смены цвета.");
    println!("[setup_torus] Нажмите 'a' для автоматической смены цвета.");
    println!("[setup_torus] Нажмите Escape для выхода.");
}

/// Система обработки ввода с клавиатуры.
/// Аналог функции keyboardFunc() в оригинальной программе.
/// В Bevy используется система ButtonInput<KeyCode>, которая автоматически
/// опрашивает состояние клавиатуры каждый кадр.
fn keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut palette: ResMut<ColorPalette>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>, With<LabObject>>,
    mut auto_timer: ResMut<AutoColorTimer>,
    state: Res<State<AutoColorState>>,
    mut next_state: ResMut<NextState<AutoColorState>>,
) {
    // Обработка нажатия клавиши «Пробел» — переключение цвета вручную.
    // Аналог: if (key == ' ') в keyboardFunc().
    if keyboard.just_pressed(KeyCode::Space) {
        let new_color = palette.next_color();
        let c = new_color.to_srgba();
        println!(
            "[keyboard] Цвет изменён (пробел): индекс = {}, цвет = ({:.1}, {:.1}, {:.1})",
            palette.current_index, c.red, c.green, c.blue
        );
        update_torus_color(&new_color, &mut materials, &query);
    }

    // Обработка нажатия 'a' — включение/выключение автоматической смены цвета.
    // Аналог: организация таймера через glutTimerFunc(20, simulation, 0).
    if keyboard.just_pressed(KeyCode::KeyA) {
        let new_state = match **state {
            AutoColorState::Off => AutoColorState::On,
            AutoColorState::On => AutoColorState::Off,
        };
        println!(
            "[keyboard] Автоматическая смена цвета: {:?}",
            new_state
        );
        next_state.set(new_state);
        if let AutoColorState::On = new_state {
            auto_timer.0.reset();
        }
    }

    // Обработка Escape — выход из программы.
    // В оригинальной программе выход происходил при закрытии окна.
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("[keyboard] Выход по Escape.");
    }
}

/// Система автоматической смены цвета (работает каждый кадр).
/// Аналог функции simulation() + glutTimerFunc(20, simulation, 0).
fn auto_color_system(
    time: Res<Time>,
    mut timer: ResMut<AutoColorTimer>,
    mut palette: ResMut<ColorPalette>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>, With<LabObject>>,
    state: Res<State<AutoColorState>>,
) {
    // Система работает только если автоматическая смена включена
    if *state == AutoColorState::Off {
        return;
    }

    // tick() возвращает true, если таймер завершился.
    // Аналог: подсчёт вызовов simulation() для определения,
    // прошла ли 1 секунда (50 вызовов по 20 мс).
    if timer.0.tick(time.delta()).just_finished() {
        let new_color = palette.next_color();
        let c = new_color.to_srgba();
        println!(
            "[auto_color] Автоматическая смена: индекс = {}, цвет = ({:.1}, {:.1}, {:.1})",
            palette.current_index, c.red, c.green, c.blue
        );
        update_torus_color(&new_color, &mut materials, &query);
    }
}

/// Система плавного вращения тора.
/// Делает сцену более наглядной и демонстрирует обновление
/// каждый кадр (аналог glutPostRedisplay + вызов display()).
fn rotation_system(time: Res<Time>, mut query: Query<&mut Transform, With<LabObject>>) {
    for mut transform in &mut query {
        // Вращаем тор вокруг оси Y
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

/// Вспомогательная функция: обновляет цвет материала тора.
/// Аналог: glColor3f(r, g, b) в оригинальной программе.
fn update_torus_color(
    color: &Color,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    query: &Query<&MeshMaterial3d<StandardMaterial>, With<LabObject>>,
) {
    for material_handle in query.iter() {
        if let Some(mut material) = materials.get_mut(material_handle.id()) {
            material.base_color = *color;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ПЛАГИНЫ (Plugins) — модули, расширяющие возможности App
// ═══════════════════════════════════════════════════════════════════════════════

/// Плагин, инкапсулирующий всю логику лабораторной работы №1.
/// Использование плагинов — recommended-паттерн в Bevy,
/// позволяющий организовать код в логические модули.
struct Lab01Plugin;

impl Plugin for Lab01Plugin {
    fn build(&self, app: &mut App) {
        // Вставляем ресурсы (аналог глобальных переменных в C++)
        app.insert_resource(ColorPalette::new());
        app.insert_resource(AutoColorTimer(Timer::from_seconds(
            1.0, // Цвет меняется каждую 1 секунду
            TimerMode::Repeating,
        )));

        // Добавляем состояния (State) для управления автоматической сменой
        app.init_state::<AutoColorState>();

        // Регистрируем стартовую систему (аналог glutInit + создание окна)
        app.add_systems(Startup, setup_torus);

        // Регистрируем системы, выполняемые каждый кадр (Update)
        app.add_systems(
            Update,
            (
                keyboard_input_system,
                auto_color_system,
                rotation_system,
            ),
        );
    }
}

/// Состояние для включения/выключения автоматической смены цвета.
/// В Bevy States позволяют переключать группы систем.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum AutoColorState {
    #[default]
    Off,
    On,
}

// ═══════════════════════════════════════════════════════════════════════════════
// ТОЧКА ВХОДА (Entry Point)
// ═══════════════════════════════════════════════════════════════════════════════

/// Главная функция программы.
/// Структура App::new() .add_plugins(DefaultPlugins) .add_plugins(Lab01Plugin) .run()
/// эквивалентна последовательности вызовов в оригинальной программе:
///   glutInit(&argc, argv);
///   glutCreateWindow("Laba_01");
///   glutDisplayFunc(display);
///   glutKeyboardFunc(keyboardFunc);
///   glutMainLoop();
fn main() {
    App::new()
        // DefaultPlugins добавляют: окно, рендерер, обработку ввода,
        // систему ресурсов, таймер и другие базовые функции движка.
        .add_plugins(DefaultPlugins)
        // Настройка цвета фона: аналог glClearColor(0.22, 0.88, 0.11, 1.0)
        // В Bevy 0.19 используется ресурс ClearColor (обёртка вокруг Color).
        .insert_resource(ClearColor(Color::srgb(0.22, 0.88, 0.11)))
        // Добавляем наш плагин со всей логикой лабораторной работы
        .add_plugins(Lab01Plugin)
        // Запуск приложения (аналог glutMainLoop — бесконечный цикл)
        .run();
}
