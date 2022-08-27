use super::*;

impl Model {
    pub fn gun_aim(&mut self, gun_id: Id, target: Position) {
        let config = &self.assets.config;
        if let Some(gun) = self.guns.get_mut(&gun_id) {
            if let Some(human) = gun.attached_human.and_then(|id| self.humans.get(&id)) {
                if target.direction(&human.position, config.arena_size).len()
                    <= config.gun_orbit_radius
                {
                    // Aim at the host
                    gun.aiming_at_host = true;
                    gun.rotation =
                        Rotation::new(gun.position.direction(&target, config.arena_size).arg());
                } else {
                    gun.aiming_at_host = false;
                    gun.rotation =
                        Rotation::new(human.position.direction(&target, config.arena_size).arg());
                }
            } else {
                gun.aiming_at_host = false;
                gun.rotation =
                    Rotation::new(gun.position.direction(&target, config.arena_size).arg());
            }
        }
    }

    pub fn gun_shoot(&mut self, gun_id: Id, heavy: bool) {
        let config = &self.assets.config;
        if let Some(gun) = self.guns.get_mut(&gun_id) {
            let speed = if heavy {
                if let Some(human) = gun
                    .attached_human
                    .take()
                    .and_then(|id| self.humans.get_mut(&id))
                {
                    // Unattach from human killing them with high recoil
                    human.holding_gun = None;
                    human.knock_out_timer = Some(config.human_knockout_time);
                    config.gun_recoil_attached_speed
                } else {
                    // Shoot with high recoil
                    config.gun_heavy_recoil_speed
                }
            } else {
                config.gun_recoil_speed
            };
            let direction = gun.rotation.direction();
            // Apply recoil
            gun.velocity += -direction * speed;

            // Spawn projectile
            let offset = match &gun.collider {
                Collider::Aabb { size } => gun.rotation.direction() * size.x / Coord::new(2.0),
            };
            let projectile = Projectile {
                id: self.id_gen.next(),
                lifetime: config.projectile_lifetime,
                position: gun.position.shifted(offset, config.arena_size),
                velocity: direction * config.gun_shoot_speed,
                collider: Collider::Aabb {
                    size: vec2(0.5, 0.5).map(Coord::new),
                },
            };
            self.projectiles.insert(projectile);
        }
    }
}
