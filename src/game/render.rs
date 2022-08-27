use crate::camera_torus::CameraTorus2d;

use super::*;

mod field;
mod ui;
mod util;
mod world;

use util::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw_world(framebuffer);
        self.draw_ui(framebuffer);
    }
}
