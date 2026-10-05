use bevy::prelude::*;

use crate::graphic_object::GraphicObject;

/// Палитра цветов тора.
#[derive(Component, Debug)]
pub struct ColorPalette {
    /// Список цветов палитры, первый — начальный цвет тора.
    colors: Vec<Color>,
    /// Индекс выбранного цвета в списке.
    selected_index: usize,
    /// Накопленная доля шага для плавной смены цвета.
    accumulated_steps: f32,
}

/// Автоматическая смена цветов: включается и выключается клавишей A.
#[derive(Resource, Default)]
pub struct AutoColorChange {
    pub enabled: bool,
}

/// Общие цвета сцены.
const PALETTE_COLORS: [Color; 4] = [
    Color::srgb(1.0, 0.0, 0.0), // красный
    Color::srgb(0.0, 0.0, 1.0), // синий
    Color::srgb(1.0, 1.0, 0.0), // жёлтый
    Color::srgb(0.0, 1.0, 0.0), // зелёный
];

/// Скорость автоматической смены цветов, шагов палитры в секунду.
const AUTO_STEPS_PER_SEC: f32 = 0.8;

/// Плагин для управления тороидальными объектами сцены.
pub struct TorusPlugin;

impl Plugin for TorusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AutoColorChange>()
            .add_systems(PostStartup, setup_torus_palettes)
            .add_systems(
                Update,
                (update_toruses, auto_change_colors, update_torus_colors).chain(),
            );
    }
}

impl ColorPalette {
    /// Палитра тора: начинается с его начального цвета, затем идут общие цвета
    /// сцены. Список у тора с номером `torus_number` начинается с цвета под этим
    /// номером, поэтому при одинаковых индексах торы имеют разные цвета.
    pub fn new(start_color: Color, torus_number: usize) -> Self {
        let colors: Vec<Color> = std::iter::once(start_color)
            .chain(
                (0..PALETTE_COLORS.len())
                    .map(|i| PALETTE_COLORS[(i + torus_number) % PALETTE_COLORS.len()]),
            )
            .collect();

        Self {
            colors,
            selected_index: 0,
            accumulated_steps: 0.0,
        }
    }

    /// Выбранный цвет палитры.
    pub fn selected_color(&self) -> Color {
        self.colors[self.selected_index]
    }

    /// Увеличивает индекс выбранного цвета на единицу по кругу.
    fn select_next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.colors.len();
    }

    /// Накапливает шаги и увеличивает индекс на целое число шагов.
    fn advance(&mut self, steps: f32) {
        self.accumulated_steps += steps;

        while self.accumulated_steps >= 1.0 {
            self.accumulated_steps -= 1.0;
            self.select_next();
        }
    }
}

/// Создаёт каждому тору собственную палитру с его начальным цветом.
fn setup_torus_palettes(mut commands: Commands, toruses: Query<(Entity, &GraphicObject)>) {
    for (torus_number, (entity, object)) in toruses.iter().enumerate() {
        commands
            .entity(entity)
            .insert(ColorPalette::new(object.to_color(), torus_number));
    }
}

/// Клавиша A включает и выключает автоматическую смену цветов.
fn update_toruses(keyboard: Res<ButtonInput<KeyCode>>, mut auto: ResMut<AutoColorChange>) {
    if keyboard.just_pressed(KeyCode::KeyA) {
        auto.enabled = !auto.enabled;
    }
}

/// Включён авторежим — увеличивает индекс цвета в палитре каждого тора.
fn auto_change_colors(
    time: Res<Time>,
    auto: Res<AutoColorChange>,
    mut palettes: Query<&mut ColorPalette>,
) {
    if !auto.enabled {
        return;
    }

    // Одинаковое число шагов для всех торов — индексы растут одновременно.
    let steps = time.delta_secs() * AUTO_STEPS_PER_SEC;

    for mut palette in &mut palettes {
        palette.advance(steps);
    }
}

/// Применяет выбранный цвет палитры к материалу тора.
fn update_torus_colors(
    mut materials: ResMut<Assets<StandardMaterial>>, // ресурс всех материалов
    toruses: Query<(&ColorPalette, &MeshMaterial3d<StandardMaterial>)>, // палитра и материал
) {
    for (palette, material_handle) in &toruses {
        if let Some(mut material) = materials.get_mut(&material_handle.0) {
            material.base_color = palette.selected_color();
        }
    }
}
