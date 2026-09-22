use bevy::prelude::*;

/// Графический объект сцены
#[derive(Component, Debug, Clone)]
pub struct GraphicObject {
    pub position: Vec3,
    pub angle_deg: f32,
    pub color: Vec3,
}

impl GraphicObject {
    pub fn new(position: Vec3, angle_deg: f32, color: Vec3) -> Self {
        Self {
            position,
            angle_deg,
            color,
        }
    }

    pub fn to_transform(&self) -> Transform {
        let angle_rad = -self.angle_deg.to_radians();
        Transform::from_translation(self.position).with_rotation(Quat::from_rotation_y(angle_rad))
    }

    /// Преобразование цвета `(r, g, b)` в цвет Bevy.
    pub fn to_color(&self) -> Color {
        Color::srgb(self.color.x, self.color.y, self.color.z)
    }
}


pub fn spawn_graphic_object(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    object: GraphicObject,
) -> Entity {
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
        .id()
}
