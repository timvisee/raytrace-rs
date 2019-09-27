use crate::color::Color;

// TODO: use some set of predefined materials
// lazy_static! {
//     pub static ref AIR: Material = Material::build().color(*WHITE).albedo(0.0).build().unwrap();
//     pub static ref WATER: Material = Material::build().color(*WHITE).albedo(0.5).build().unwrap();
//     pub static ref GLASS: Material = Material::build()
//         .color(*WHITE)
//         .albedo(0.18)
//         .surface(Surface::Transparent {
//             index: 1.5,
//             transparency: 1.0,
//         })
//         .build()
//         .unwrap();
//     pub static ref METAL: Material = Material::build()
//         .color(Color::new(0.5, 0.5, 0.5))
//         .albedo(0.25)
//         .surface(Surface::Specular { reflectivity: 0.7 })
//         .build()
//         .unwrap();
// }

/// Material type for an entity.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Material {
    /// Base material color.
    pub color: Color,

    /// Material albedo value.
    pub albedo: f32,

    /// Material surface type.
    #[serde(default)]
    pub surface: Surface,
}

/// Surface type for a material.
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Surface {
    /// A diffuse surface.
    Diffuse,

    /// A specular/reflective surface.
    Specular {
        /// Should be in `(0,1)`, 0 is not reflective, 1 is fully reflective.
        reflectivity: f32,
    },

    /// A transparent surface.
    Transparent {
        /// Refractive index.
        ///
        /// The Snell's Law refractive index for this surface material.
        // TODO: what kind of index is this?
        index: f32,

        /// Transparency.
        ///
        /// Should be in `(0,1)`, 0 is opaque, 1 is fully transparent.
        transparency: f32,
    },
}

impl Default for Surface {
    fn default() -> Self {
        Self::Diffuse
    }
}
