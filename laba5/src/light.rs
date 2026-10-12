use bevy::prelude::*;

/// Ресурс источника света сцены.
///
/// Хранит параметры единственного источника света в терминах модели Фонга
/// и умеет преобразовывать их в компоненты и ресурсы движка Bevy.
#[derive(Resource, Debug, Clone)]
pub struct SceneLight {
    /// Позиция источника в глобальной системе координат.
    position: Vec3,
    /// Цвет диффузной и зеркальной составляющих.
    color: Color,
    /// Световой поток, люмены.
    intensity: f32,
    /// Радиус действия источника.
    range: f32,
    /// Цвет фоновой составляющей.
    ambient_color: Color,
    /// Яркость фоновой составляющей, кд/м².
    ambient_brightness: f32,
}

impl SceneLight {
    /// Конструктор: белый источник в указанной позиции.
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            color: Color::WHITE,
            intensity: 6_000_000.0,
            range: 100.0,
            ambient_color: Color::WHITE,
            ambient_brightness: 200.0,
        }
    }

    /// Задание цвета источника.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Задание интенсивности источника (световой поток, люмены).
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Задание фонового освещения сцены.
    pub fn set_ambient(&mut self, color: Color, brightness: f32) {
        self.ambient_color = color;
        self.ambient_brightness = brightness;
    }

    /// Позиция источника.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Преобразование в компонент `PointLight`.
    pub fn point_light(&self) -> PointLight {
        PointLight {
            color: self.color,
            intensity: self.intensity,
            range: self.range,
            ..default()
        }
    }

    /// Преобразование в компонент `Transform`.
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position)
    }

    /// Преобразование в ресурс `GlobalAmbientLight`.
    pub fn ambient_light(&self) -> GlobalAmbientLight {
        GlobalAmbientLight {
            color: self.ambient_color,
            brightness: self.ambient_brightness,
            ..default()
        }
    }
}

/// Маркерный компонент сущности источника света, управляемой ресурсом [`SceneLight`].
#[derive(Component)]
pub struct SceneLightMarker;

/// Создаёт сущность источника света при старте приложения.
pub fn spawn_scene_light(commands: &mut Commands, light: &Res<SceneLight>) {
    commands.spawn((
        PointLight {
            color: light.color,
            intensity: light.intensity,
            range: light.range,
            ..default()
        },
        light.transform(),
        SceneLightMarker,
    ));
}

/// Переносит параметры ресурса [`SceneLight`] в сущность источника и
/// в ресурс фонового освещения.
pub fn apply_light(
    light: Res<SceneLight>,
    mut query: Query<(&mut PointLight, &mut Transform), With<SceneLightMarker>>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    if !light.is_changed() {
        return;
    }

    for (mut point_light, mut transform) in &mut query {
        *point_light = light.point_light();
        *transform = light.transform();
    }

    *ambient = light.ambient_light();
}