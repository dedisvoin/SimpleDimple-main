use bevy::prelude::*;

/// Графический объект сцены: привязка модели к точке размещения.
///
/// Аналог класса `GraphicObject` из оригинальной лабораторной на C++:
/// хранит позицию в мировой системе координат, угол поворота вокруг `Oy`
/// и цвет объекта. Матрица модели вычисляется методом [`to_transform`](Self::to_transform).
#[derive(Component, Debug, Clone)]
pub struct GraphicObject {
    /// Позиция объекта в world space.
    pub position: Vec3,
    /// Угол поворота «носика» модели вокруг оси `Oy` (градусы).
    pub angle_deg: f32,
    /// Цвет объекта в формате `(r, g, b)`.
    pub color: Vec3,
}

impl GraphicObject {
    /// Конструктор графического объекта.
    pub fn new(position: Vec3, angle_deg: f32, color: Vec3) -> Self {
        Self {
            position,
            angle_deg,
            color,
        }
    }

    /// Расчёт матрицы модели (`Transform`).
    ///
    /// Матрица модели задаёт положение (`position`) и ориентацию (`angle_deg`)
    /// объекта в составе сцены. Отрицательный знак перед углом приводит вращение
    /// к направлению «по часовой стрелке», как в оригинальной лабораторной.
    /// «Носик» модели в локальной СК направлен вдоль `+X`, поэтому угол подбирается
    /// так, чтобы локальная `+X` указывала в центр сцены.
    pub fn to_transform(&self) -> Transform {
        let angle_rad = -self.angle_deg.to_radians();
        Transform::from_translation(self.position).with_rotation(Quat::from_rotation_y(angle_rad))
    }

    /// Преобразование цвета `(r, g, b)` в цвет Bevy.
    pub fn to_color(&self) -> Color {
        Color::srgb(self.color.x, self.color.y, self.color.z)
    }
}

/// Порождение сущности графического объекта на сцене (аналог `draw` в C++).
///
/// Создаёт тор (модель), материал заданного цвета и связывает их с сущностью,
/// к которой прикреплён компонент `GraphicObject` с вычисленным `Transform`.
/// Над тором помещается текстовая метка `number` (дочерняя сущность), поэтому
/// при движении объекта подпись следует за ним.
pub fn spawn_graphic_object(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    object: GraphicObject,
    number: usize,
) {
    // Модель тора один раз кладётся в Assets<Mesh> и переиспользуется всеми объектами.
    let torus_mesh = meshes.add(Torus::new(1.0, 0.4));

    // Материал с цветом объекта.
    let material = materials.add(StandardMaterial {
        base_color: object.to_color(),
        ..default()
    });

    // Создаём сущность с мешем, материалом, матрицей модели и данными объекта.
    commands
        .spawn((
            Mesh3d(torus_mesh),
            MeshMaterial3d(material),
            object.to_transform(),
            object,
        ))
        .with_children(|parent| {
            // Подпись — номер объекта — выше тора в локальных координатах родителя.
            parent.spawn((
                Text2d::new(number.to_string()),
                TextFont::from_font_size(40.0),
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 1.9, 0.0),
            ));
        });
}
