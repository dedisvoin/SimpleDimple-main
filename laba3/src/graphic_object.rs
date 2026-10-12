use bevy::prelude::*;

/// Графический объект сцены
#[derive(Component, Debug, Clone)]
pub struct GraphicObject {
    pub position: Vec3,
    pub angle_deg: f32,
    pub color: Vec3,
    pub color_index: usize,
}

impl GraphicObject {
    pub fn new(position: Vec3, angle_deg: f32, color: Vec3) -> Self {
        Self {
            position,
            angle_deg,
            color,
            color_index: 0, // будет выставлен при спавне
        }
    }

    pub fn to_transform(&self) -> Transform {
        let angle_rad = -self.angle_deg.to_radians();
        Transform::from_translation(self.position).with_rotation(Quat::from_rotation_y(angle_rad))
    }

    pub fn to_color(&self) -> Color {
        Color::srgb(self.color.x, self.color.y, self.color.z)
    }

    /// Переход к следующему цвету палитры.
    pub fn advance_color(&mut self, palette_len: usize) {
        if palette_len > 0 {
            self.color_index = (self.color_index + 1) % palette_len;
        }
    }
}

pub fn spawn_graphic_object(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mut object: GraphicObject,
    color_index: usize,
    palette: &[Vec3],
) -> Entity {
    let torus_mesh = meshes.add(Torus::new(1.0, 0.4));

    // Присваиваем индивидуальный цвет из палитры.
    object.color_index = color_index;
    object.color = palette[color_index];

    let material = materials.add(StandardMaterial {
        base_color: object.to_color(),
        ..default()
    });

    commands
        .spawn((
            Mesh3d(torus_mesh),
            MeshMaterial3d(material),
            object.to_transform(),
            object,
        ))
        .id()
}