use std::fmt::{self};

use serde::{Deserialize, Serialize};

pub const FRAME_RATE: u64 = 20;

// Add your new renderer here
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Renderers {
    RedWave,
    Template,
    Snow,
    EnderLogo,
    SpaceFight,
    RainbowWave,
    Mario,
    JWST,
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
            Renderers::RainbowWave => write!(f, "Rainbow Wave"),
            Renderers::Mario => write!(f, "Mario"),
            Renderers::JWST => write!(f, "JWST"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Renderer {
    pub name: Renderers,
    pub enabled: bool,

    pub link: String,
    // pub subtitle: String,
    // pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct RendererStrings {
    pub renderers: Vec<Renderer>,
}

pub fn renderer_strings() -> Vec<Renderer> {
    let renderers = [
        (Renderers::RedWave, "https://cdn.discordapp.com/attachments/444005079410802699/924071697852821515/RedWaves.mp4"),
        (Renderers::Template, "https://cdn.discordapp.com/attachments/444005079410802699/924071698360303656/Template.mp4"),
        (Renderers::Snow, "https://cdn.discordapp.com/attachments/444005079410802699/924071698066714664/Snow.mp4"),
        (Renderers::EnderLogo, "https://cdn.discordapp.com/attachments/444005079410802699/924071697211080704/EnderLogo.mp4"),
        (Renderers::SpaceFight, "https://cdn.discordapp.com/attachments/444005079410802699/924071696955215872/spacefight.mp4"),
        (Renderers::RainbowWave, "https://cdn.discordapp.com/attachments/444005079410802699/924071697387245598/RainbowWave.mp4"),
        (Renderers::Mario, "https://cdn.discordapp.com/attachments/444005079410802699/924071697630527568/Mario.mp4"),
        (Renderers::JWST, "https://cdn.discordapp.com/attachments/444005079410802699/924071697630527568/Mario.mp4"),
    ];

    renderers
        .iter()
        .map(|r| {
            let renderer = r.clone();
            Renderer {
                name: renderer.0,
                enabled: false,
                link: renderer.1.to_string(),
            }
        })
        .collect()
}
