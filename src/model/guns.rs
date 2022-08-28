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

    pub fn gun_shoot(&mut self, gun_id: Id, heavy: bool, events: &mut Vec<Event>) {
        let config = &self.assets.config;

        if let Some(gun) = self.guns.get_mut(&gun_id) {
            enum ShotType {
                Normal,
                Heavy,
                Kill,
            }

            let shot_type = if heavy {
                if gun.attached_human.is_some() {
                    // Unattach from human killing them with high recoil
                    ShotType::Kill
                } else {
                    // Shoot with high recoil
                    ShotType::Heavy
                }
            } else {
                ShotType::Normal
            };
            let (speed, bullets) = match shot_type {
                ShotType::Normal => (config.gun_recoil_speed, 1),
                ShotType::Heavy => (config.gun_heavy_recoil_speed, config.gun_heavy_bullets),
                ShotType::Kill => (config.gun_recoil_attached_speed, config.gun_heavy_bullets),
            };
            if gun.ammo < bullets {
                return;
            }
            gun.ammo -= bullets;

            let direction = gun.rotation.direction();
            // Apply recoil
            gun.velocity += -direction * speed;

            // Spawn projectile
            match shot_type {
                ShotType::Normal => {
                    let offset = match &gun.collider {
                        Collider::Aabb { size } => {
                            gun.rotation.direction() * size.x / Coord::new(2.0)
                        }
                    };
                    let projectile = Projectile {
                        id: self.id_gen.next(),
                        lifetime: config.gun_shoot_lifetime,
                        position: gun.position.shifted(offset, config.arena_size),
                        velocity: direction * config.gun_shoot_speed,
                        collider: Collider::Aabb {
                            size: vec2(0.5, 0.5).map(Coord::new),
                        },
                    };
                    self.projectiles.insert(projectile);
                }
                ShotType::Heavy => {
                    for i in 0..config.gun_heavy_bullets {
                        let rotation = gun.rotation
                            + Rotation::new(
                                Coord::new(i as f32 / (config.gun_heavy_bullets - 1) as f32 - 0.5)
                                    * config.gun_heavy_angle,
                            );
                        let offset = match &gun.collider {
                            Collider::Aabb { size } => {
                                rotation.direction() * size.x / Coord::new(2.0)
                            }
                        };
                        let projectile = Projectile {
                            id: self.id_gen.next(),
                            lifetime: config.gun_heavy_lifetime,
                            position: gun.position.shifted(offset, config.arena_size),
                            velocity: direction * config.gun_heavy_speed,
                            collider: Collider::Aabb {
                                size: vec2(0.5, 0.5).map(Coord::new),
                            },
                        };
                        self.projectiles.insert(projectile);
                    }
                }
                ShotType::Kill => {
                    if let Some(human) = gun
                        .attached_human
                        .take()
                        .and_then(|id| self.humans.get_mut(&id))
                    {
                        human.holding_gun = None;
                        human.is_alive = false;
                    }
                }
            }

            events.push(Event::Shoot {
                position: gun.position,
                direction,
            });
        }
    }
}
