use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.process_movement(delta_time);
        self.process_collisions(delta_time);
        self.process_deaths(delta_time);
    }

    pub fn gun_shoot(&mut self, gun_id: Id, direction: Vec2<Coord>) {
        if let Some(gun) = self.guns.get_mut(&gun_id) {
            let direction = direction.normalize_or_zero();
            // Apply recoil
            gun.velocity += -direction * Coord::new(GUN_RECOIL_SPEED);

            // Spawn projectile
            let projectile = Projectile {
                id: self.id_gen.next(),
                lifetime: Time::new(PROJECTILE_LIFETIME),
                position: gun.position,
                velocity: direction * Coord::new(GUN_SHOOT_SPEED),
                collider: Collider::Aabb {
                    size: vec2(1.0, 1.0).map(Coord::new),
                },
            };
            self.projectiles.insert(projectile);
        }
    }

    fn process_movement(&mut self, delta_time: Time) {
        // Move guns
        for gun in &mut self.guns {
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
        // Check for collisions with projectiles
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
