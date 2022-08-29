use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            // geng::Event::MouseDown { button, .. } => {
            //     self.control_mode = ControlMode::Mouse;
            //     let heavy = match button {
            //         geng::MouseButton::Left => false,
            //         geng::MouseButton::Right => true,
            //         geng::MouseButton::Middle => return,
            //     };
            //     self.model.send(Message::Shoot { heavy });
            // }
            geng::Event::TouchStart { touches } => {
                let touch = Touch {
                    time: self.game_time,
                    initial: touches.clone(),
                    current: touches,
                };
                self.touch_start(touch);
            }
            geng::Event::TouchMove { touches } => {
                if let Some(touch) = &mut self.touch {
                    touch.current = touches;
                    self.touch_move();
                }
            }
            geng::Event::TouchEnd { .. } => {
                if let Some(touch) = self.touch.take() {
                    self.touch_end(touch);
                }
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
                self.play_sound(&self.assets.shoot, position);
            }
            Event::ProjectileCollide { position, velocity } => {
                self.spawn_particles(
                    position,
                    velocity * Coord::new(0.1),
                    Time::new(0.5),
                    5,
                    Rgba::RED,
                    vec2(0.2, 0.2).map(Coord::new),
                );
                self.play_sound(&self.assets.hit, position);
            }
        }
    }

    fn touch_start(&mut self, touch: Touch) {
        self.control_mode = ControlMode::Touch;
        self.touch = Some(touch);
        self.touch_move();
    }

    fn touch_move(&mut self) {
        if let Some(touch) = &mut self.touch {
            let middle = touch
                .current
                .iter()
                .map(|point| point.position)
                .fold(Vec2::ZERO, Vec2::add)
                / touch.current.len() as f64;
            let world = self
                .camera
                .screen_to_world(
                    self.framebuffer_size.map(|x| x as f32),
                    middle.map(|x| x as f32),
                )
                .map(Coord::new);
            self.model.send(Message::Aim { target: world })
        }
    }

    fn touch_end(&mut self, touch: Touch) {
        match touch.initial[..] {
            [_] => {
                if let [_] = touch.current[..] {
                    if self.game_time - touch.time < Time::new(0.2) {
                        self.model.send(Message::Shoot { heavy: false });
                    }
                }
            }
            [_, _] => {
                if let [_, _] = touch.current[..] {
                    if self.game_time - touch.time < Time::new(0.2) {
                        self.model.send(Message::Shoot { heavy: true });
                    }
                }
            }
            _ => {}
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
