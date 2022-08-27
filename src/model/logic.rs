mod collisions;
mod guns;
mod humans;
mod movement;

use super::*;

pub struct Logic<'a> {
    pub delta_time: Time,
    pub model: &'a mut Model,
}

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        let mut logic = Logic {
            delta_time,
            model: self,
        };
        logic.process();
    }
}

impl Logic<'_> {
    pub fn process(&mut self) {
        self.process_guns();
        self.process_humans();
        self.process_movement();
        self.process_collisions();
        self.process_deaths();
        self.check_state();
    }

    fn process_deaths(&mut self) {
        // Check for human deaths
        self.model.humans.retain(|human| human.is_alive);

        // Check for projectiles "deaths" (collisions or lifetime)
        for projectile in &mut self.model.projectiles {
            projectile.lifetime -= self.delta_time;
        }
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }

    fn check_state(&mut self) {
        match &mut self.model.state {
            GameState::InProgress => {
                if self.model.humans.is_empty() {
                    // The game is finished
                    self.model.state = GameState::Finished {
                        time_left: self.model.assets.config.game_restart_delay,
                    };
                }
            }
            GameState::Finished { time_left } => {
                *time_left -= self.delta_time;
                if *time_left <= Time::ZERO {
                    // Start the game again
                    self.model.restart();
                }
            }
        }
    }
}
