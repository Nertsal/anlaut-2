use super::*;

impl Model {
    pub fn gun_aim(&mut self, gun_id: Id, target: Position) {
        if let Some(gun) = self.guns.get_mut(&gun_id) {
            if let Some(human) = gun.attached_human.and_then(|id| self.humans.get(&id)) {
                if (human.position - target).len()
                    <= Coord::new(self.assets.config.gun_orbit_radius)
                {
                    // Aim at the host
                    gun.aiming_at_host = true;
                    gun.rotation = Rotation::new((target - gun.position).arg());
                } else {
                    gun.aiming_at_host = false;
                    gun.rotation = Rotation::new((target - human.position).arg());
                }
            } else {
                gun.aiming_at_host = false;
                gun.rotation = Rotation::new((target - gun.position).arg());
            }
        }
    }

    pub fn gun_shoot(&mut self, gun_id: Id, release: bool) {
        let config = &self.assets.config;
        if let Some(gun) = self.guns.get_mut(&gun_id) {
            if release {
                // Unattach from human
                if let Some(human) = gun
                    .attached_human
                    .take()
                    .and_then(|id| self.humans.get_mut(&id))
                {
                    human.holding_gun = None;
                    human.knock_out_timer = Some(Time::new(config.human_knockout_time));
                }
            }
            let direction = gun.rotation.direction();
            // Apply recoil
            gun.velocity += -direction * Coord::new(config.gun_recoil_speed);

            // Spawn projectile
            let offset = match &gun.collider {
                Collider::Aabb { size } => gun.rotation.direction() * size.x / Coord::new(2.0),
            };
            let projectile = Projectile {
                id: self.id_gen.next(),
                lifetime: Time::new(config.projectile_lifetime),
                position: gun.position + offset,
                velocity: direction * Coord::new(config.gun_shoot_speed),
                collider: Collider::Aabb {
                    size: vec2(0.5, 0.5).map(Coord::new),
                },
            };
            self.projectiles.insert(projectile);
        }
    }
}
