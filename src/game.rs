use geng::Draw2d;

use super::*;
use crate::model::*;

const TICKS_PER_SECOND: f64 = 60.0;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    model: simple_net::Remote<Model>,
    next_update: f64,
    player_id: Id,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, player_id: Id, model: simple_net::Remote<Model>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            model,
            next_update: 0.0,
            player_id,
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None);
        // TODO
    }

    fn handle_event(&mut self, _event: geng::Event) {}

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
