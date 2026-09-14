use bevy::prelude::*;

/// Структура для хранения палитры цветов.
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
                Color::srgb(0.0, 1.0, 1.0),
                Color::srgb(1.0, 0.5, 0.0),
                Color::srgb(0.0, 0.0, 1.0),
                Color::BLACK,
                Color::WHITE
            ],
            current_index: 0,
            automatic: false,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    /// Получаем текущий цвет
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

fn main() {
    App::new()
        // Добавляем стандартные плагины
        .add_plugins(DefaultPlugins)

        // Устанавливаем цвет заливки окна
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))

        // Ресурсом добавляем нашу палитру цветов
        .insert_resource(ColorPalette::new())

        // Добавляем функцию setup как инициализирующую
        .add_systems(Startup, setup)

        // Добавляем 4 функции выполняемые каждый кадр
        .add_systems(
            Update,
            (
                listen_keyboard,
                automatic_color_system,
                update_torus_color_system,
                torus_rotate_system,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands, // команды для манипулирования сценой
    mut meshes: ResMut<Assets<Mesh>>, // ресурс хранящий меши
    mut materials: ResMut<Assets<StandardMaterial>>, // ресурс хранящий материалы
) {
    // Создаём геометрию тора.
    let torus_mesh = meshes.add(Torus::new(1.0, 0.4));

    // Создаём материал тора.
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        ..default()
    });

    // Создаём Entity тора.
    commands.spawn((
        Mesh3d(torus_mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        TorusMarker, // специальный маркер компонент чтобы найти наш тор среди других мешей
    ));

    // Создаём камеру.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Добавляем направленный источник света.
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}


fn listen_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut palette: ResMut<ColorPalette>,
) {
    /////////////////////////////////////////////////////////
    // Пробел переключает следующий цвет.
    /////////////////////////////////////////////////////////
    if keyboard.just_pressed(KeyCode::Space) {
        palette.next_color();
    }

    /////////////////////////////////////////////////////////
    // 1-5 переключают цвет по индексу.
    /////////////////////////////////////////////////////////
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

    /////////////////////////////////////////////////////////
    // A переключает автоматическую систему смены цвета.
    /////////////////////////////////////////////////////////
    if keyboard.just_pressed(KeyCode::KeyA) {
        palette.automatic = !palette.automatic;
    }
}

/// Система смены цвета
fn automatic_color_system(
    time: Res<Time>,
    mut palette: ResMut<ColorPalette>,
) {
    /*
     * ! Цвет меняется если автоматическая система включена и таймер завершился в этом кадре
    */
    if palette.automatic && palette.timer.tick(time.delta()).just_finished() {
        palette.next_color();
    }
}


/// Обновляет цвет торуса на основе текущего цвета палитры.
fn update_torus_color_system(
    palette: Res<ColorPalette>,
    mut materials: ResMut<Assets<StandardMaterial>>, // Ресурс всех материалов стандартного типа
    query: Query<&MeshMaterial3d<StandardMaterial>, With<TorusMarker>>, // Запрос на материалы торусов
) {
    for material_handle in &query {
        // пытаемся получить получить материал по его дескриптору (.0)
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = palette.current_color(); // устанавливаем цвет
        }
    }
}


/// Маркер-компонент указывает, какой тор должен вращаться.
#[derive(Component)]
struct TorusMarker;

const TORUS_ROTATE_SPEED: f32 = 3.; /// Скорость вращения тора в радианах

/// Система вращения тора
fn torus_rotate_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<TorusMarker>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(TORUS_ROTATE_SPEED * time.delta_secs());
        transform.rotate_y(TORUS_ROTATE_SPEED * time.delta_secs());
    }
}
