use super::*;

impl Game {
    pub fn draw_field(&self, framebuffer: &mut ugli::Framebuffer) {
        let unit_quad = unit_quad(self.geng.ugli());
        ugli::draw(
            framebuffer,
            &*self.assets.shaders.field,
            ugli::DrawMode::TriangleFan,
            &unit_quad,
            (
                ugli::uniforms! {
                    u_time: self.game_time.as_f32(),
                    cellSize : 3.0,
                    u_color_1 : vec3(0.02, 0.02, 0.02),
                    u_color_2 : vec3(0.07, 0.07, 0.07),
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
