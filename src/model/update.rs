use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        // Move projectiles
        for projectile in &mut self.projectiles {
            projectile.position += projectile.velocity * delta_time;
        }
    }
}
