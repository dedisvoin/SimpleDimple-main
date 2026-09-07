use bevy::prelude::*;

#[derive(Resource)]
struct ColorPalette {
    colors: Vec<Color>,
    current_index: usize,
    automatic: bool,
    timer: Timer,
}

impl ColorPalette {
    fn new() -> Self {
        Self {
            colors: vec![
                Color::BLACK,
                Color::WHITE,
                Color::srgb(0.0, 0.2, 1.0),
                Color::srgb(1.0, 0.0, 0.0),
                Color::srgb(0.6, 0.0, 1.0),
            ],
            current_index: 2,
            automatic: false,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    fn current_color(&self) -> Color {
        self.colors[self.current_index]
    }

    fn next_color(&mut self) {
        self.current_index = (self.current_index + 1) % self.colors.len();
    }

    fn set_color(&mut self, index: usize) {
        if index < self.colors.len() {
            self.current_index = index;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(ColorPalette::new())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_color_system,
                automatic_color_system,
                update_torus_color_system,
                rotate_torus_system,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Создаём геометрию тора.
    let torus = meshes.add(Torus::new(1.0, 0.4));

    // Создаём материал тора.
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.2, 1.0),
        ..default()
    });

    // Создаём Entity тора.
    commands.spawn((
        Mesh3d(torus),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Создаём камеру.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Создаём источник света.
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            ..default()
        },
        Transform::from_xyz(3.0, 4.0, 5.0),
    ));
}

fn keyboard_color_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut palette: ResMut<ColorPalette>,
) {
    // Пробел переключает следующий цвет.
    if keyboard.just_pressed(KeyCode::Space) {
        palette.next_color();
    }

    // Клавиши 1-5 выбирают цвет напрямую.
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

    // Клавиша A включает или выключает автоматическую смену.
    if keyboard.just_pressed(KeyCode::KeyA) {
        palette.automatic = !palette.automatic;
    }
}

fn automatic_color_system(
    time: Res<Time>,
    mut palette: ResMut<ColorPalette>,
) {
    if palette.automatic && palette.timer.tick(time.delta()).just_finished() {
        palette.next_color();
    }
}

fn update_torus_color_system(
    palette: Res<ColorPalette>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    for material_handle in &query {
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = palette.current_color();
        }
    }
}

fn rotate_torus_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(1.0 * time.delta_secs());
    }
}