use std::fs;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;

/// Вершина полигональной сетки
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    /// Геометрические координаты в системе координат модели.
    pub position: [f32; 3],

    /// Вектор нормали к поверхности.
    pub normal: [f32; 3],

    /// Текстурные координаты.
    pub uv: [f32; 2],
}

/// Ссылка на атрибуты одной вершины полигона в obj-файле.
#[derive(Debug, Clone, Copy)]
struct FaceIndex {
    position: usize,
    uv: Option<usize>,
    normal: Option<usize>,
}

/// Полигональная сетка, загруженная из obj-файла.
pub struct ObjMesh {
    /// Массив вершин полигональной сетки.
    vertices: Vec<Vertex>,
}

impl ObjMesh {
    /// Загрузка полигональной сетки из obj-файла.
    pub fn load(path: &str) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        // Массив полигонов
        let mut faces: Vec<Vec<FaceIndex>> = Vec::new();

        for (index, line) in text.lines().enumerate() {
            let line_number = index + 1;
            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("v") => positions.push(parse_floats(parts, path, line_number)?),
                Some("vn") => normals.push(parse_floats(parts, path, line_number)?),
                Some("vt") => {
                    // Третья текстурная координата, если она есть, не используется.
                    let [u, v, _] = parse_floats::<3>(parts.chain(["0"]), path, line_number)?;
                    uvs.push([u, v]);
                }
                Some("f") => {
                    let face = parts
                        .map(|token| parse_face_index(token, path, line_number))
                        .collect::<Result<Vec<_>, _>>()?;
                    if face.len() < 3 {
                        return Err(format!(
                            "{path}:{line_number}: в полигоне меньше трёх вершин"
                        ));
                    }
                    faces.push(face);
                }
                // Комментарии, пустые строки, группы и прочие строки пропускаются.
                _ => {}
            }
        }

        let mut vertices = Vec::new();

        for face in &faces {
            // Многоугольник разбивается на треугольники веером
            // (0, 1, 2), (0, 2, 3), (0, 3, 4) ...
            for k in 1..face.len() - 1 {
                let triangle = [face[0], face[k], face[k + 1]];

                let corners = triangle
                    .map(|corner| get(&positions, Some(corner.position), "v", path))
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;

                // вычисляем нормаль треугольника, если в нормали не заданы.
                let flat_normal = triangle_normal(corners[0], corners[1], corners[2]);

                for (corner, position) in triangle.iter().zip(corners) {
                    let normal = corner
                        .normal
                        .map(|index| get(&normals, Some(index), "vn", path))
                        .transpose()?
                        .unwrap_or(flat_normal);
                    let uv = corner
                        .uv
                        .map(|index| get(&uvs, Some(index), "vt", path))
                        .transpose()?
                        .unwrap_or([0.0, 0.0]);
                    vertices.push(Vertex {
                        position,
                        normal,
                        uv,
                    });
                }
            }
        }

        println!(
            "Загружен меш {path}: координат {}, нормалей {}, текстурных координат {}, \
             полигонов {}, вершин {}",
            positions.len(),
            normals.len(),
            uvs.len(),
            faces.len(),
            vertices.len()
        );

        Ok(Self { vertices })
    }

    /// Преобразование сетки в меш движка Bevy.
    pub fn to_bevy_mesh(&self) -> Mesh {
        let positions: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.position).collect();
        let normals: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.normal).collect();

        let uvs: Vec<[f32; 2]> = self
            .vertices
            .iter()
            .map(|v| [v.uv[0], 1.0 - v.uv[1]])
            .collect();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}

/// Считывает `N` вещественных чисел из итератора.
fn parse_floats<'a, const N: usize>(
    mut parts: impl Iterator<Item = &'a str>,
    path: &str,
    line_number: usize,
) -> Result<[f32; N], String> {
    let mut result = [0.0_f32; N];

    for slot in &mut result {
        let token = parts
            .next()
            .ok_or_else(|| format!("{path}:{line_number}: недостаточно чисел"))?;
        *slot = token
            .parse()
            .map_err(|error| format!("{path}:{line_number}: {error}"))?;
    }

    Ok(result)
}

/// Разбирает одну группу f-строки: `v`, `v/vt`, `v//vn` или `v/vt/vn`.
fn parse_face_index(token: &str, path: &str, line_number: usize) -> Result<FaceIndex, String> {
    let mut parts = token.split('/');

    let position = parts.next().unwrap_or_default();
    let uv = parts.next().filter(|value| !value.is_empty());
    let normal = parts.next().filter(|value| !value.is_empty());

    Ok(FaceIndex {
        position: parse_index(position, path, line_number)?,
        uv: uv
            .map(|value| parse_index(value, path, line_number))
            .transpose()?,
        normal: normal
            .map(|value| parse_index(value, path, line_number))
            .transpose()?,
    })
}

/// Переводит номер атрибута из obj-файла (с единицы) в индекс массива (с нуля).
fn parse_index(token: &str, path: &str, line_number: usize) -> Result<usize, String> {
    let index: usize = token
        .parse()
        .map_err(|error| format!("{path}:{line_number}: {error}"))?;

    index
        .checked_sub(1)
        .ok_or_else(|| format!("{path}:{line_number}: номер атрибута начинается с единицы"))
}

/// Берёт элемент массива по необязательному индексу.
fn get<T: Copy>(array: &[T], index: Option<usize>, kind: &str, path: &str) -> Result<T, String> {
    let index = index.ok_or_else(|| format!("{path}: отсутствует индекс атрибута «{kind}»"))?;

    array
        .get(index)
        .copied()
        .ok_or_else(|| format!("{path}: индекс {kind} {} вне диапазона", index + 1))
}

/// Нормаль треугольника через векторное произведение двух его сторон.
fn triangle_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let ab = Vec3::from_array(b) - Vec3::from_array(a);
    let ac = Vec3::from_array(c) - Vec3::from_array(a);
    let normal = ab.cross(ac);

    if normal.length_squared() > 0.0 {
        normal.normalize().to_array()
    } else {
        [0.0, 1.0, 0.0]
    }
}
