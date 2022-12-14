use geng::Draw2d;

use super::*;
use crate::model::*;

mod handle_event;
mod interpolation;
pub mod render;
mod shake;
mod update;

use interpolation::*;
use render::*;
use shake::*;

const TICKS_PER_SECOND: f64 = 60.0;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: Render,
    volume: f64,
    control_mode: ControlMode,
    touch: Option<Touch>,
    model: Connection,
    game_time: Time,
    next_update: f64,
    camera_position: Position,
    camera_target_position: Position,
    camera_shake: Shake,
    framebuffer_size: Vec2<usize>,
    frame_texture: ugli::Texture,
    new_texture: ugli::Texture,
    player_id: PlayerId,
    spectating: Option<PlayerId>,
    transition_explosion_radius: Option<Coord>,
}

enum ControlMode {
    Mouse,
    Touch,
}

#[derive(Debug, Clone)]
pub struct Touch {
    pub time: Time,
    pub initial: Vec<geng::TouchPoint>,
    pub current: Vec<geng::TouchPoint>,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Position,
    pub velocity: Vec2<Coord>,
    pub lifetime: Time,
    pub size: Vec2<Coord>,
    pub color: Rgba<f32>,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub position: Position,
    pub velocity: Vec2<Coord>,
    pub lifetime: Time,
    pub size: Coord,
    pub color: Rgba<f32>,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, player_id: PlayerId, model: Connection) -> Self {
        let render = Render::new(geng, assets);
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            volume: 0.5,
            control_mode: ControlMode::Mouse,
            touch: None,
            model,
            game_time: Time::ZERO,
            next_update: 0.0,
            camera_position: Position::ZERO,
            camera_target_position: Position::ZERO,
            camera_shake: Shake::new(),
            framebuffer_size: vec2(1, 1),
            frame_texture: ugli::Texture::new_uninitialized(geng.ugli(), vec2(1, 1)),
            new_texture: ugli::Texture::new_uninitialized(geng.ugli(), vec2(1, 1)),
            player_id,
            spectating: None,
            transition_explosion_radius: Some(render.camera.fov * Coord::new(2.0)),
            render,
        }
    }

    fn play_sound(&self, sound: &geng::Sound, pos: Position) {
        let mut effect = sound.effect();
        let distance = pos
            .distance(
                &self.render.camera.center,
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

            for event in self.model.update(delta_time) {
                self.handle_model_event(event);
            }
        }
    }
}
