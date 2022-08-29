use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { button, .. } => {
                self.control_mode = ControlMode::Mouse;
                let heavy = match button {
                    geng::MouseButton::Left => false,
                    geng::MouseButton::Right => true,
                    geng::MouseButton::Middle => return,
                };
                self.model.send(Message::Shoot { heavy });
            }
            geng::Event::KeyDown { key } => match key {
                geng::Key::Left => self.cycle_spectator(-1),
                geng::Key::Right => self.cycle_spectator(1),
                _ => {}
            },
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
            Event::ProjectileCollide {
                position,
                velocity,
                powerup,
            } => {
                let color = powerup_color(powerup.as_ref());
                self.spawn_particles(
                    position,
                    velocity * Coord::new(0.1),
                    Time::new(0.5),
                    5,
                    color,
                    vec2(0.2, 0.2).map(Coord::new),
                );
                self.play_sound(&self.assets.hit, position);
            }
        }
    }

    fn cycle_spectator(&mut self, delta: isize) {
        let spectating = match &mut self.spectating {
            Some(s) => s,
            None => return,
        };

        let model = self.model.get();
        let get_player_pos = |player: &Player| match &player.state {
            PlayerState::Gun { gun_id } => {
                model.guns.get(gun_id).map(|gun| (player.id, gun.position))
            }
            _ => None,
        };

        let mut players: Vec<_> = model.players.iter().filter_map(get_player_pos).collect();
        players.sort_by_key(|(id, _)| *id);

        let current = players
            .iter()
            .enumerate()
            .find(|(_, (id, _))| *id == *spectating);
        let current = match current {
            Some((i, _)) => {
                let mut i = i as isize + delta;
                let len = players.len() as isize;
                while i < 0 {
                    i += len;
                }
                while i >= len {
                    i -= len;
                }
                i as usize
            }
            None => 0,
        };
        *spectating = players[current].0;
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
        if self.game_time - touch.time < Time::new(0.2) {
            self.quick_touch(touch);
        }
    }

    fn quick_touch(&mut self, touch: Touch) {
        match touch.initial[..] {
            [point] => {
                if self.spectating.is_some() {
                    let delta =
                        (point.position.x - self.framebuffer_size.x as f64).signum() as isize;
                    self.cycle_spectator(delta);
                }
                self.model.send(Message::Shoot { heavy: false });
            }
            [_, _] => {
                self.model.send(Message::Shoot { heavy: true });
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
