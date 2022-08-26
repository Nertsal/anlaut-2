use super::*;

impl Logic<'_> {
    pub fn process_movement(&mut self) {
        // Move guns
        for gun in &mut self.model.guns {
            if let Some(human) = gun.attached_human.and_then(|id| self.model.humans.get(&id)) {
                // Attached to a human
                let mult = if gun.aiming_at_host {
                    -Coord::ONE
                } else {
                    Coord::ONE
                };
                let offset = gun.rotation.direction() * Coord::new(GUN_ORBIT_RADIUS) * mult;
                gun.position = human.position + offset;
                gun.velocity = Vec2::ZERO;
                continue;
            }
            gun.velocity -=
                gun.velocity.clamp_len(..=Coord::ONE) * Coord::new(GUN_FRICTION) * self.delta_time;
            gun.position += gun.velocity * self.delta_time;
        }

        // Move projectiles
        for projectile in &mut self.model.projectiles {
            projectile.position += projectile.velocity * self.delta_time;
        }
    }
}
