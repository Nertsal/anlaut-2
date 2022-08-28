use super::*;

const CAMERA_INTERPOLATION: f32 = 0.5;

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        self.interpolate(delta_time);

        let model = self.model.get();
        let config = &model.assets.config;

        // Particles
        for particle in &mut self.particles {
            particle
                .position
                .shift(particle.velocity * delta_time, config.arena_size);
            particle.lifetime -= delta_time;
        }
        self.particles
            .retain(|particle| particle.lifetime > Time::ZERO);

        // Camera target position
        if let Some(player) = model.players.get(&self.player_id) {
            if let PlayerState::Gun { gun_id } = &player.state {
                if let Some(gun) = model.guns.get(gun_id) {
                    self.camera_target_position = gun.position;
                }
            }
        }

        // Camera interpolation
        self.camera.center.shift(
            self.camera
                .center
                .direction(&self.camera_target_position, config.arena_size)
                / Coord::new(CAMERA_INTERPOLATION)
                * delta_time,
            config.arena_size,
        );

        // Aim
        if let ControlMode::Mouse = self.control_mode {
            let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
            let mouse_pos = self
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
