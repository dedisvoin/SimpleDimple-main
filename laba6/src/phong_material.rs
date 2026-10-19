use std::fs;

use bevy::prelude::*;

/// Параметры материала в терминах модели освещения Фонга.
///
/// Каждый цвет задаётся четвёркой `(r, g, b, a)` в диапазоне `[0.0, 1.0]`.
/// Материал не используется движком напрямую: метод [`PhongMaterial::to_standard_material`]
/// переводит его в стандартный материал Bevy.
#[derive(Debug, Clone, PartialEq)]
pub struct PhongMaterial {
    /// Диффузная отражающая способность (основной цвет объекта), Md.
    pub diffuse: Vec4,
    /// Зеркальная отражающая способность (сила блика), Ms.
    pub specular: Vec4,
    /// Цвет самосвечения, Me.
    pub emission: Vec4,
    /// Степень отполированности (концентрация блика), p.
    pub shininess: f32,
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            diffuse: Vec4::new(0.8, 0.8, 0.8, 1.0),  // основной цвет
            specular: Vec4::new(0.5, 0.5, 0.5, 1.0), // цвет блика
            emission: Vec4::new(0.0, 0.0, 0.0, 1.0), // самосвечение
            shininess: 32.0,                         // концентрация блика
        }
    }
}

impl PhongMaterial {
    /// Загрузка параметров материала из текстового файла.
    ///
    /// Формат файла: по одному параметру в строке — имя и значения,
    /// разделённые пробелами. Строки, начинающиеся с `#`, — комментарии.
    pub fn load(path: &str) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
        let mut material = Self::default();

        for (index, line) in text.lines().enumerate() {
            let line = line.trim();

            // Пустые строки и комментарии пропускаются.
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line_number = index + 1;
            let mut parts = line.split_whitespace();
            let key = parts.next().unwrap_or_default();

            let values = parts
                .map(str::parse::<f32>)
                .collect::<Result<Vec<f32>, _>>()
                .map_err(|error| format!("{path}:{line_number}: {error}"))?;

            match (key, values.as_slice()) {
                ("diffuse", &[r, g, b, a]) => material.diffuse = Vec4::new(r, g, b, a),
                ("specular", &[r, g, b, a]) => material.specular = Vec4::new(r, g, b, a),
                ("emission", &[r, g, b, a]) => material.emission = Vec4::new(r, g, b, a),
                ("shininess", &[p]) => material.shininess = p,
                _ => return Err(format!("{path}:{line_number}: неверная строка «{line}»")),
            }
        }

        Ok(material)
    }

    /// Перевод параметров материала в стандартный материал Bevy.
    pub fn to_standard_material(&self) -> StandardMaterial {
        let d = self.diffuse;
        let e = self.emission;

        // Сила блика — наибольшая из компонент зеркальной составляющей.
        let specular_strength = self.specular.x.max(self.specular.y).max(self.specular.z);

        StandardMaterial {
            base_color: Color::srgba(d.x, d.y, d.z, d.w),
            emissive: LinearRgba::rgb(e.x, e.y, e.z),
            reflectance: specular_strength.clamp(0.0, 1.0),
            perceptual_roughness: shininess_to_roughness(self.shininess),
            metallic: 0.0,
            ..default()
        }
    }
}

/// Перевод степени отполированности `p` в шероховатость по модели фонга.
fn shininess_to_roughness(shininess: f32) -> f32 {
    let alpha = (2.0 / (shininess.max(0.0) + 2.0)).sqrt();
    alpha.sqrt().clamp(0.089, 1.0)
}
