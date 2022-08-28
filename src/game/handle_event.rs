use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let _mouse_pos = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
            .map(Coord::new);

        #[allow(clippy::single_match)]
        match event {
            geng::Event::MouseDown { button, .. } => {
                let heavy = match button {
                    geng::MouseButton::Left => false,
                    geng::MouseButton::Right => true,
                    geng::MouseButton::Middle => return,
                };
                self.model.send(Message::Shoot { heavy });
            }
            _ => {}
        }
    }

    pub fn handle_model_event(&mut self, event: Event) {
        match event {
            Event::Shoot {
                position,
                direction,
            } => {
                self.spawn_particles(
                    position,
                    direction * Coord::new(7.0),
                    Time::new(0.2),
                    10,
                    Rgba::RED,
                    vec2(0.2, 0.2).map(Coord::new),
                );
            }
            Event::ProjectileCollide { position } => {
                self.spawn_particles(
                    position,
                    Vec2::ZERO,
                    Time::new(1.0),
                    10,
                    Rgba::RED,
                    vec2(0.2, 0.2).map(Coord::new),
                );
            }
        }
    }

    fn spawn_particles(
        &mut self,
        position: Position,
        velocity: Vec2<Coord>,
        lifetime: Time,
        amount: usize,
        color: Rgba<f32>,
        size: Vec2<Coord>,
    ) {
        let config = &self.model.get().assets.config;
        let mut rng = global_rng();
        for _ in 0..amount {
            let shift = vec2(rng.gen_range(-0.1..=0.1), rng.gen_range(-0.1..=0.1)).map(Coord::new);
            let position = position.shifted(shift, config.arena_size);
            let shift = vec2(rng.gen_range(-0.1..=0.1), rng.gen_range(-0.1..=0.1)).map(Coord::new);
            let angle = Coord::new(rng.gen_range(-0.5..=0.5));
            let velocity = (velocity + shift).rotate(angle);
            let particle = Particle {
                position,
                velocity,
                lifetime,
                size,
                color,
            };
            self.particles.push(particle);
        }
    }
}
