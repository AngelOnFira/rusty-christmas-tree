use std::fmt;

use serde::{Deserialize, Serialize};

pub const FRAME_RATE: u64 = 20;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Renderers {
    RedLines,
}

impl fmt::Display for Renderers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Renderers::RedLines => write!(f, "Red Lines"),
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
    let renderers = [Renderers::RedLines];

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
