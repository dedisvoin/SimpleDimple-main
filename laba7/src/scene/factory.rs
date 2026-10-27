//! Фабрика игровых объектов и загрузка их внешнего описания.

use std::{collections::HashMap, fs, path::Path};

use bevy::prelude::{Assets, Color, Handle, LinearRgba, Mesh, Resource, StandardMaterial, Vec3};
use serde::Deserialize;

use crate::logic::Cell;

use super::{BoardLayout, GameObject, GraphicObject, mesh::ObjMesh};

/// Тип внешнего вида игрового объекта.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameObjectType {
    /// Лёгкий объект.
    LightObject,
    /// Тяжёлый объект.
    HeavyObject,
    /// Граничный объект.
    BorderObject,
    PlayerObject,
}

impl GameObjectType {
    /// Сопоставляет клетке тип внешнего вида; пустая клетка не создаёт объект.
    pub fn from_cell(cell: Cell) -> Option<Self> {
        match cell {
            Cell::Empty => None,
            Cell::Light => Some(Self::LightObject),
            Cell::Heavy => Some(Self::HeavyObject),
            Cell::Border => Some(Self::BorderObject),
        }
    }
}

/// Фабрика, хранящая общие меши и материалы игровых объектов.
#[derive(Resource)]
pub struct GameObjectFactory {
    /// Меш для каждого типа объекта.
    meshes: HashMap<GameObjectType, Handle<Mesh>>,
    /// Материал для каждого типа объекта.
    materials: HashMap<GameObjectType, Handle<StandardMaterial>>,
    /// Раскладка карты для перевода клеток в координаты сцены.
    layout: BoardLayout,
}

/// Описание внешнего вида одного типа объекта в JSON.
#[derive(Debug, Deserialize)]
struct GameObjectDescription {
    /// Путь к файлу меша.
    mesh: String,
    /// Параметры материала.
    material: MaterialDescription,
}

/// Параметры материала из JSON-файла.
#[derive(Debug, Deserialize)]
struct MaterialDescription {
    /// Цвет в пространстве sRGB.
    base_color: [f32; 4],
    /// Металличность от 0.0 до 1.0.
    metallic: f32,
    /// Шероховатость от 0.0 до 1.0.
    perceptual_roughness: f32,
    /// Сила блика диэлектрика.
    reflectance: f32,
    /// Самосвечение в линейном RGB; по умолчанию выключено.
    #[serde(default)]
    emissive: [f32; 3],
}

impl GameObjectFactory {
    /// Читает JSON-описание, загружает меши и создаёт материалы объектов.
    ///
    /// Повторяющиеся пути мешей используют один общий handle. Ошибки чтения и
    /// разбора JSON возвращаются с путём к файлу.
    pub fn init(
        meshes_assets: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        layout: BoardLayout,
        path: impl AsRef<Path>,
    ) -> Result<Self, String> {
        let path = path.as_ref();
        let text =
            fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
        let descriptions: HashMap<GameObjectType, GameObjectDescription> =
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;

        let mut meshes = HashMap::new();
        let mut materials_by_type = HashMap::new();
        let mut meshes_by_path: HashMap<String, Handle<Mesh>> = HashMap::new();

        for (kind, description) in descriptions {
            let mesh = if let Some(mesh) = meshes_by_path.get(&description.mesh) {
                mesh.clone()
            } else {
                let obj = ObjMesh::load(&description.mesh)?;
                let mesh = meshes_assets.add(obj.to_bevy_mesh());
                meshes_by_path.insert(description.mesh.clone(), mesh.clone());
                mesh
            };

            let material_description = description.material;
            let material = materials.add(StandardMaterial {
                base_color: Color::srgba(
                    material_description.base_color[0],
                    material_description.base_color[1],
                    material_description.base_color[2],
                    material_description.base_color[3],
                ),
                metallic: material_description.metallic,
                perceptual_roughness: material_description.perceptual_roughness,
                reflectance: material_description.reflectance,
                emissive: LinearRgba::rgb(
                    material_description.emissive[0],
                    material_description.emissive[1],
                    material_description.emissive[2],
                ),
                ..Default::default()
            });

            meshes.insert(kind, mesh);
            materials_by_type.insert(kind, material);
        }

        Ok(Self {
            meshes,
            materials: materials_by_type,
            layout,
        })
    }

    /// Создаёт игровой объект заданного типа в указанной клетке.
    pub fn create(
        &self,
        kind: GameObjectType,
        position: bevy::prelude::IVec2,
    ) -> Result<GameObject, String> {
        let (Some(mesh), Some(material)) = (self.meshes.get(&kind), self.materials.get(&kind))
        else {
            return Err(format!("тип объекта {kind:?} не описан в JSON-файле"));
        };

        let graphic_object = GraphicObject::new(Vec3::ZERO, 0.0, mesh.clone(), material.clone());
        Ok(GameObject::new(graphic_object, position, self.layout))
    }
}
