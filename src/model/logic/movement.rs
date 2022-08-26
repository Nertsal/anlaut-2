use super::*;

impl Logic<'_> {
    pub fn process_movement(&mut self) {
        let config = &self.model.assets.config;

        // Move humans
        for human in &mut self.model.humans {
            human.position += human.velocity * self.delta_time;
        }

        // Move guns
        for gun in &mut self.model.guns {
            if let Some(human) = gun.attached_human.and_then(|id| self.model.humans.get(&id)) {
                // Attached to a human
                let mult = if gun.aiming_at_host {
                    -Coord::ONE
                } else {
                    Coord::ONE
                };
                let offset = gun.rotation.direction() * Coord::new(config.gun_orbit_radius) * mult;
                gun.position = human.position + offset;
                gun.velocity = Vec2::ZERO;
                continue;
            }
            gun.velocity -= gun.velocity.clamp_len(..=Coord::ONE)
                * Coord::new(config.gun_friction)
                * self.delta_time;
            gun.position += gun.velocity * self.delta_time;
        }

        // Move projectiles
        for projectile in &mut self.model.projectiles {
            projectile.position += projectile.velocity * self.delta_time;
        }
    }
}
