use super::*;

impl Model {
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
}
