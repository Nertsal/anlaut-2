use super::*;

const CAMERA_INTERPOLATION: f32 = 0.5;

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        self.interpolate(delta_time);

        // Touch update
        if let Some(touch) = &mut self.touch {
            let elapsed = self.game_time - touch.time;
            if elapsed > Time::new(0.25) {
                let touch = self.touch.take().unwrap();
                self.hold_touch(touch);
            }
        }

        let model = self.model.get();
        let config = &model.assets.config;

        // Spectator validation
        if let Some(player) = model.players.get(&self.player_id) {
            if let PlayerState::Spectator = player.state {
            } else {
                self.spectating = None;
            }
        }

        // Particles
        for particle in &mut self.render.particles {
            particle
                .position
                .shift(particle.velocity * delta_time, config.arena_size);
            particle.lifetime -= delta_time;
        }
        self.render
            .particles
            .retain(|particle| particle.lifetime > Time::ZERO);

        // Texts
        for text in &mut self.render.texts {
            text.position
                .shift(text.velocity * delta_time, config.arena_size);
            text.lifetime -= delta_time;
        }
        self.render.texts.retain(|text| text.lifetime > Time::ZERO);

        // Camera target position
        if let Some(player) = model.players.get(&self.player_id) {
            match &player.state {
                PlayerState::Gun { gun_id } => {
                    if let Some(gun) = model.guns.get(gun_id) {
                        self.camera_target_position = gun.position;
                    }
                }
                PlayerState::Spectator => {
                    let get_player_pos = |player: &Player| match &player.state {
                        PlayerState::Gun { gun_id } => {
                            model.guns.get(gun_id).map(|gun| (player.id, gun.position))
                        }
                        _ => None,
                    };
                    let spectating = self
                        .spectating
                        .and_then(|id| model.players.get(&id).and_then(get_player_pos))
                        .or_else(|| model.players.iter().find_map(get_player_pos));
                    match spectating {
                        Some((player, position)) => {
                            self.camera_target_position = position;
                            self.spectating = Some(player);
                        }
                        None => self.spectating = None,
                    }
                }
                _ => {}
            }
        }

        // Camera interpolation
        self.camera_position.shift(
            self.camera_position
                .direction(&self.camera_target_position, config.arena_size)
                / Coord::new(CAMERA_INTERPOLATION)
                * delta_time,
            config.arena_size,
        );

        // Shake
        self.camera_shake.update(delta_time);
        self.render.camera.center = self
            .camera_position
            .shifted(self.camera_shake.offset(), config.arena_size);

        // Aim
        if let ControlMode::Mouse = self.control_mode {
            let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
            let mouse_pos = self
                .render
                .camera
                .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
                .map(Coord::new);
            self.model.send(Message::Aim { target: mouse_pos });
        }
    }

    fn interpolate(&mut self, delta_time: Time) {
        let model = self.model.get();
        let to_interpolate = itertools::chain!(
            model
                .projectiles
                .iter()
                .map(|proj| (proj.id, proj.position, proj.velocity)),
            model
                .humans
                .iter()
                .map(|human| (human.id, human.position, human.velocity)),
            model
                .guns
                .iter()
                .map(|gun| (gun.id, gun.position, gun.velocity))
        );
        for (id, target_pos, target_vel) in to_interpolate {
            let interpolated = self
                .render
                .interpolated_positions
                .entry(id)
                .or_insert_with(|| Interpolation::new(target_pos, target_vel));
            interpolated.update(
                delta_time,
                target_pos,
                target_vel,
                model.assets.config.arena_size,
            );
        }
    }
}
