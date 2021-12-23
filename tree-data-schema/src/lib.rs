use std::fmt::{self, write};

use serde::{Deserialize, Serialize};

pub const FRAME_RATE: u64 = 20;

// Add your new renderer here
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Renderers {
    RedWave,
    Template,
    Snow,
    EnderLogo,
    SpaceFight
}

impl fmt::Display for Renderers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Add a new display for your variant here
        match self {
            Renderers::RedWave => write!(f, "Red Wave"),
            Renderers::Template => write!(f, "Template"),
            Renderers::Snow => write!(f, "Snow"),
            Renderers::EnderLogo => write!(f, "Ender Logo"),
            Renderers::SpaceFight => write!(f, "Space Fight"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Renderer {
    pub name: Renderers,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RendererStrings {
    pub renderers: Vec<Renderer>,
}

pub fn renderer_strings() -> Vec<Renderer> {
    let renderers = [Renderers::RedWave];

    renderers
        .iter()
        .map(|r| {
            let renderer = r.clone();
            Renderer {
                name: renderer,
                enabled: false,
            }
        })
        .collect()
}
