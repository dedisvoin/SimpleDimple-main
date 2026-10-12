use bevy::prelude::*;

/// Графический объект сцены.
///
/// Хранит позицию, угол поворота и разделяемую ссылку на материал.
/// Материал может использоваться несколькими объектами — это агрегация.
#[derive(Component, Debug, Clone)]
pub struct GraphicObject {
    pub position: Vec3,
    pub angle_deg: f32,
    /// Разделяемая ссылка на материал объекта.
    pub material: Handle<StandardMaterial>,
}

impl GraphicObject {
    pub fn new(position: Vec3, angle_deg: f32, material: Handle<StandardMaterial>) -> Self {
        Self {
            position,
            angle_deg,
            material,
        }
    }

    pub fn to_transform(&self) -> Transform {
        let angle_rad = -self.angle_deg.to_radians();
        Transform::from_translation(self.position).with_rotation(Quat::from_rotation_y(angle_rad))
    }
}

/// Создаёт сущность графического объекта на сцене.
///
/// Меш тора и материал передаются как разделяемые ссылки: одни и те же
/// ресурсы могут использоваться несколькими объектами.
pub fn spawn_graphic_object(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    object: GraphicObject,
) -> Entity {
    let transform = object.to_transform();
    let material = object.material.clone();

    println!(
        "Создан объект: позиция {:?}, угол {:.1}°",
        object.position, object.angle_deg
    );

    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            transform,
            object,
        ))
        .id()
}