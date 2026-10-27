use bevy::prelude::*;

/// Графический объект сцены.
///
/// Хранит матрицу модели (позицию и угол поворота) и разделяемые ссылки на меш
/// и материал. Меш и материал могут использоваться несколькими объектами — это
/// агрегация.
#[derive(Component, Debug, Clone)]
pub struct GraphicObject {
    /// Позиция объекта в глобальной системе координат.
    pub position: Vec3,

    /// Угол поворота вокруг оси Oy, в градусах.
    pub angle_deg: f32,

    /// Разделяемая ссылка на меш (геометрическую форму).
    pub mesh: Handle<Mesh>,

    /// Разделяемая ссылка на материал.
    pub material: Handle<StandardMaterial>,
}

impl GraphicObject {
    pub fn new(
        position: Vec3,
        angle_deg: f32,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            position,
            angle_deg,
            mesh,
            material,
        }
    }

    /// Матрица модели: перенос в позицию объекта и поворот вокруг оси Oy.
    pub fn to_transform(&self) -> Transform {
        let angle_rad = -self.angle_deg.to_radians();
        Transform::from_translation(self.position).with_rotation(Quat::from_rotation_y(angle_rad))
    }

    /// Набор компонентов, по которым движок выводит графический объект.
    pub fn render_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>, Transform) {
        (
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
            self.to_transform(),
        )
    }
}

/// Создаёт сущность графического объекта на сцене.
///
/// Графический объект может использоваться самостоятельно: так выводится
/// основание сцены, не участвующее в игровой логике.
#[allow(dead_code)]
pub fn spawn_graphic_object(commands: &mut Commands, object: GraphicObject) -> Entity {
    let components = object.render_components();

    println!(
        "Создан графический объект: позиция {:?}, угол {:.1}°",
        object.position, object.angle_deg
    );

    commands.spawn((object, components)).id()
}
