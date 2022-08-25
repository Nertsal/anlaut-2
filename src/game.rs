mod handle_event;
mod render;

use geng::Draw2d;

use super::*;
use crate::model::*;

const TICKS_PER_SECOND: f64 = 60.0;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    model: net::Remote<Model>,
    next_update: f64,
    camera: geng::Camera2d,
    framebuffer_size: Vec2<usize>,
    player_id: PlayerId,
}

impl Game {
    pub fn new(
        geng: &Geng,
        assets: &Rc<Assets>,
        player_id: PlayerId,
        model: net::Remote<Model>,
    ) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            model,
            next_update: 0.0,
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                fov: 20.0,
                rotation: 0.0,
            },
            framebuffer_size: vec2(1, 1),
            player_id,
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), None);
        self.draw(framebuffer)
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.handle_event(event)
    }

    fn update(&mut self, delta_time: f64) {
        self.next_update -= delta_time;
        while self.next_update < 0.0 {
            let delta_time = 1.0 / TICKS_PER_SECOND;
            self.next_update += delta_time;

            let delta_time = Time::new(delta_time as f32);
            // TODO

            for _event in self.model.update() {
                // TODO
            }
        }
    }
}
