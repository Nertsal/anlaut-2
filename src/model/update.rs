use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.process_humans(delta_time);
        self.process_movement(delta_time);
        self.process_collisions(delta_time);
        self.process_deaths(delta_time);
    }

    pub fn gun_aim(&mut self, gun_id: Id, target: Position) {
        if let Some(gun) = self.guns.get_mut(&gun_id) {
            if let Some(human) = gun.attached_human.and_then(|id| self.humans.get(&id)) {
                if (human.position - target).len() <= Coord::new(GUN_ORBIT_RADIUS) {
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
        if let Some(gun) = self.guns.get_mut(&gun_id) {
            if release {
                // Unattach from human
                if let Some(human) = gun
                    .attached_human
                    .take()
                    .and_then(|id| self.humans.get_mut(&id))
                {
                    human.holding_gun = None;
                    human.knock_out_timer = Some(Time::new(HUMAN_KNOCKOUT_TIME));
                }
            }
            let direction = gun.rotation.direction();
            // Apply recoil
            gun.velocity += -direction * Coord::new(GUN_RECOIL_SPEED);

            // Spawn projectile
            let offset = match &gun.collider {
                Collider::Aabb { size } => gun.rotation.direction() * size.x / Coord::new(2.0),
            };
            let projectile = Projectile {
                id: self.id_gen.next(),
                lifetime: Time::new(PROJECTILE_LIFETIME),
                position: gun.position + offset,
                velocity: direction * Coord::new(GUN_SHOOT_SPEED),
                collider: Collider::Aabb {
                    size: vec2(0.5, 0.5).map(Coord::new),
                },
            };
            self.projectiles.insert(projectile);
        }
    }

    fn process_humans(&mut self, delta_time: Time) {
        for human in &mut self.humans {
            if let Some(timer) = &mut human.knock_out_timer {
                *timer -= delta_time;
                if *timer <= Time::ZERO {
                    human.knock_out_timer = None;
                }
            }
        }
    }

    fn process_movement(&mut self, delta_time: Time) {
        // Move guns
        for gun in &mut self.guns {
            if let Some(human) = gun.attached_human.and_then(|id| self.humans.get(&id)) {
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
                gun.velocity.clamp_len(..=Coord::ONE) * Coord::new(GUN_FRICTION) * delta_time;
            gun.position += gun.velocity * delta_time;
        }

        // Move projectiles
        for projectile in &mut self.projectiles {
            projectile.position += projectile.velocity * delta_time;
        }
    }

    fn process_collisions(&mut self, _delta_time: Time) {
        // Check for projectile-human collisions
        for projectile in &mut self.projectiles {
            for human in &mut self.humans {
                if projectile
                    .collider
                    .check(&human.collider, human.position - projectile.position)
                {
                    // Collision detected -> hill the human
                    human.is_alive = false;
                    projectile.lifetime = Time::ZERO;
                    break;
                }
            }
        }

        // Check for gun-human collisions
        for gun in &mut self.guns {
            if let Some(human_id) = &gun.attached_human {
                // Check if human is still alive
                if !self
                    .humans
                    .get(human_id)
                    .map(|human| human.is_alive)
                    .unwrap_or(false)
                {
                    // Human is dead -> drop the weapon
                    let gun_id = self.humans.get_mut(human_id).unwrap().holding_gun.take();
                    assert_eq!(
                        gun_id,
                        Some(gun.id),
                        "human's holding_gun and gun's attached_human are conflicting"
                    );
                    gun.attached_human = None;
                }
                continue;
            }
            // Check for collisions
            for human in &mut self.humans {
                if !human.is_alive || human.holding_gun.is_some() || human.knock_out_timer.is_some()
                {
                    continue;
                }
                if gun
                    .collider
                    .check(&human.collider, human.position - gun.position)
                {
                    // Collision detected -> attach the gun to the human
                    human.holding_gun = Some(gun.id);
                    gun.attached_human = Some(human.id);
                    break;
                }
            }
        }
    }

    fn process_deaths(&mut self, delta_time: Time) {
        // Check for human deaths
        self.humans.retain(|human| human.is_alive);

        // Check for projectiles "deaths" (collisions or lifetime)
        for projectile in &mut self.projectiles {
            projectile.lifetime -= delta_time;
        }
        self.projectiles
            .retain(|projectile| projectile.lifetime > Hp::ZERO);
    }
}
