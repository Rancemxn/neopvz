use neopvz_core::{LOGICAL_HEIGHT, LOGICAL_WIDTH};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogicalViewport {
    pub width: u32,
    pub height: u32,
}

impl Default for LogicalViewport {
    fn default() -> Self {
        Self {
            width: LOGICAL_WIDTH,
            height: LOGICAL_HEIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteCommand {
    pub resource_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: i32,
    pub scale: f32,
    pub alpha: f32,
}

#[derive(Clone, Debug, Default)]
pub struct RenderFrame {
    pub sprites: Vec<SpriteCommand>,
}

impl RenderFrame {
    pub fn sort_for_submission(&mut self) {
        self.sprites.sort_by_key(|sprite| sprite.z);
    }
}
