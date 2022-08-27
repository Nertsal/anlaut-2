use super::*;

impl Game {
    pub fn draw_field(&self, framebuffer: &mut ugli::Framebuffer) {
        let unit_quad = ugli::VertexBuffer::new_dynamic(
            self.geng.ugli(),
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
        );

        ugli::draw(
            framebuffer,
            &*self.assets.field,
            ugli::DrawMode::TriangleFan,
            &unit_quad,
            (
                ugli::uniforms! {
                    u_time: self.game_time.as_f32(),
                    cellSize : 3.0,
                    u_color_1 : vec3(0.01, 0.01, 0.01),
                    u_color_2 : vec3(0.04, 0.04, 0.04),
                },
                geng::camera2d_uniforms(&self.camera, framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::default()),
                ..default()
            },
        );
    }
}
