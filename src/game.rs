use geng::Draw2d;

use super::*;
use crate::{camera_torus::CameraTorus2d, model::*};

mod handle_event;
mod interpolation;
mod render;
mod update;

use interpolation::*;

const TICKS_PER_SECOND: f64 = 60.0;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    volume: f64,
    control_mode: ControlMode,
    touch: Option<Touch>,
    model: net::Remote<Model>,
    particles: Vec<Particle>,
    interpolated_positions: HashMap<Id, Interpolation>,
    game_time: Time,
    next_update: f64,
    camera: CameraTorus2d,
    camera_target_position: Position,
    framebuffer_size: Vec2<usize>,
    player_id: PlayerId,
}

enum ControlMode {
    Mouse,
    Touch,
}

struct Particle {
    pub position: Position,
    pub velocity: Vec2<Coord>,
    pub lifetime: Time,
    pub size: Vec2<Coord>,
    pub color: Rgba<f32>,
}

struct Touch {
    pub time: Time,
    pub initial: Vec<geng::TouchPoint>,
    pub current: Vec<geng::TouchPoint>,
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
            volume: 0.5,
            control_mode: ControlMode::Mouse,
            touch: None,
            model,
            particles: default(),
            interpolated_positions: default(),
            game_time: Time::ZERO,
            next_update: 0.0,
            camera: CameraTorus2d {
                center: Position::ZERO,
                fov: Coord::new(30.0),
            },
            camera_target_position: Position::ZERO,
            framebuffer_size: vec2(1, 1),
            player_id,
        }
    }

    fn play_sound(&self, sound: &geng::Sound, pos: Position) {
        let mut effect = sound.effect();
        let distance = pos
            .distance(
                &self.camera.center,
                self.model.get().assets.config.arena_size,
            )
            .as_f32();
        let volume = (1.0 - (distance / 25.0).sqr()).max(0.0) as f64 * self.volume;
        effect.set_volume(volume);
        effect.play()
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.draw(framebuffer)
    }

    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyDown { key } = event {
            match key {
                geng::Key::PageDown => {
                    self.volume -= 0.1;
                }
                geng::Key::PageUp => {
                    self.volume += 0.1;
                }
                _ => {}
            }
            self.volume = self.volume.clamp(0.0, 1.0);
        }
        self.handle_event(event)
    }

    fn update(&mut self, delta_time: f64) {
        self.next_update -= delta_time;
        while self.next_update < 0.0 {
            let delta_time = 1.0 / TICKS_PER_SECOND;
            self.next_update += delta_time;

            let delta_time = Time::new(delta_time as f32);
            self.game_time += delta_time;
            self.update(delta_time);

            for event in self.model.update() {
                self.handle_model_event(event);
            }
        }
    }
}
