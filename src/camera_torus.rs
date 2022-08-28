use crate::model::{Coord, Position};
use geng::prelude::{Mat3, Vec2};
use geng::Camera2d;

pub struct CameraTorus2d {
    pub center: Position,
    pub fov: Coord,
}

impl CameraTorus2d {
    pub fn normal_camera(&self) -> Camera2d {
        Camera2d {
            center: self.center.to_world_f32(),
            fov: self.fov.as_f32(),
            rotation: 0.0,
        }
    }

    pub fn project(&self, position: Position, world_size: Vec2<Coord>) -> Vec2<Coord> {
        let center = self.center.to_world();
        center + self.center.direction(&position, world_size)
    }
}

impl geng::AbstractCamera2d for CameraTorus2d {
    fn view_matrix(&self) -> Mat3<f32> {
        self.normal_camera().view_matrix()
    }

    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat3<f32> {
        self.normal_camera().projection_matrix(framebuffer_size)
    }
}
