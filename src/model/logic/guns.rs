use super::*;

impl Logic<'_> {
    pub fn process_guns(&mut self) {
        let config = &self.model.assets.config;

        for gun in &mut self.model.guns {
            if gun.ammo >= config.gun_magazine_size {
                gun.next_reload = config.gun_reload_time;
                continue;
            }
            gun.next_reload -= self.delta_time;
            if gun.next_reload <= Time::ZERO && gun.ammo < config.gun_magazine_size {
                gun.ammo += 1;
                gun.next_reload = config.gun_reload_time;
            }
        }
    }
}
