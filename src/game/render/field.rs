use super::*;

pub fn draw_field(
    game_time: Time,
    shader: &ugli::Program,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    let unit_quad = unit_quad(geng.ugli());
    ugli::draw(
        framebuffer,
        shader,
        ugli::DrawMode::TriangleFan,
        &unit_quad,
        (
            ugli::uniforms! {
                u_time: game_time.as_f32(),
                cellSize : 3.0,
                u_color_1 : vec3(0.02, 0.02, 0.02),
                u_color_2 : vec3(0.07, 0.07, 0.07),
            },
            geng::camera2d_uniforms(camera, framebuffer.size().map(|x| x as f32)),
        ),
        ugli::DrawParameters {
            blend_mode: Some(ugli::BlendMode::default()),
            ..default()
        },
    );
}
