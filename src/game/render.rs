use crate::camera_torus::CameraTorus2d;

use super::*;

mod field;
mod ui;
mod util;
mod world;

use util::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        // Render to a temporary texture
        let mut texture = self.texture.take().unwrap_or_else(|| {
            ugli::Texture::new_uninitialized(self.geng.ugli(), framebuffer.size())
        });
        let temp_framebuffer = &mut ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut texture),
        );
        ugli::clear(temp_framebuffer, Some(Rgba::BLACK), None, None);

        // Render all the staff
        self.draw_world(temp_framebuffer);
        self.draw_ui(temp_framebuffer);

        // Do post-processing
        ugli::draw(
            temp_framebuffer,
            &*self.assets.shaders.post,
            ugli::DrawMode::TriangleFan,
            &unit_quad(self.geng.ugli()),
            (
                ugli::uniforms! {
                    u_time: self.game_time.as_f32(),
                },
                geng::camera2d_uniforms(&self.camera, framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::default()),
                ..default()
            },
        );

        // Render to the screen
        draw_2d::TexturedQuad::new(
            AABB::ZERO.extend_positive(framebuffer.size().map(|x| x as f32)),
            texture,
        )
        .draw_2d(&self.geng, framebuffer, &geng::PixelPerfectCamera);
    }
}

fn unit_quad(ugli: &Ugli) -> ugli::VertexBuffer<draw_2d::Vertex> {
    ugli::VertexBuffer::new_dynamic(
        ugli,
        vec![
            draw_2d::Vertex {
                a_pos: vec2(-1.0, -1.0),
            },
            draw_2d::Vertex {
                a_pos: vec2(-1.0, 1.0),
            },
            draw_2d::Vertex {
                a_pos: vec2(1.0, 1.0),
            },
            draw_2d::Vertex {
                a_pos: vec2(1.0, -1.0),
            },
        ],
    )
}
