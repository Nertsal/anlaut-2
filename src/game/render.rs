use crate::camera_torus::CameraTorus2d;

use super::*;

mod field;
mod ui;
pub mod util;
mod world;

use util::*;

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    pub camera: CameraTorus2d,
    pub texts: Vec<Text>,
    pub interpolated_positions: HashMap<Id, Interpolation>,
    pub particles: Vec<Particle>,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: CameraTorus2d {
                center: Position::ZERO,
                fov: Coord::new(30.0),
            },
            texts: default(),
            interpolated_positions: default(),
            particles: default(),
        }
    }

    pub fn spawn_text(&mut self, position: Position, text: String) {
        let mut rng = global_rng();

        let angle = rng.gen_range(-0.5..=0.5);
        let (sin, cos) = angle.sin_cos();
        let speed = rng.gen_range(2.0..=3.0);
        let velocity = (vec2(cos, sin) * speed).map(Coord::new);

        let text = Text {
            text,
            position,
            velocity,
            lifetime: Time::new(1.5),
            size: Coord::new(0.2),
            color: self.assets.colors.text,
        };
        self.texts.push(text);
    }
}

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        // Update textures
        if self.framebuffer_size != framebuffer.size() {
            let new = || ugli::Texture::new_uninitialized(self.geng.ugli(), framebuffer.size());
            self.frame_texture = new();
            self.new_texture = new();
        }
        self.framebuffer_size = framebuffer.size();
        let framebuffer_size = framebuffer.size().map(|x| x as f32);

        // Render to a temporary texture
        let temp_framebuffer = &mut ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.frame_texture),
        );
        ugli::clear(temp_framebuffer, Some(Rgba::BLACK), None, None);

        // Render all the staff
        let model = &*self.model.get();
        self.render.draw_world(
            self.game_time,
            model,
            temp_framebuffer,
            &mut self.new_texture,
        );
        self.render.draw_ui(model, self.model.is_local(), self.player_id, temp_framebuffer);

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
                geng::camera2d_uniforms(&self.render.camera, framebuffer_size),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::default()),
                ..default()
            },
        );

        // Render to the screen
        draw_2d::TexturedQuad::new(
            AABB::ZERO.extend_positive(framebuffer_size),
            &self.frame_texture,
        )
        .draw_2d(&self.geng, framebuffer, &geng::PixelPerfectCamera);
    }
}

pub fn unit_quad(ugli: &Ugli) -> ugli::VertexBuffer<draw_2d::Vertex> {
    ugli::VertexBuffer::new_dynamic(
        ugli,
        AABB::ZERO
            .extend_symmetric(vec2(1.0, 1.0))
            .corners()
            .into_iter()
            .map(|v| draw_2d::Vertex { a_pos: v })
            .collect(),
    )
}
