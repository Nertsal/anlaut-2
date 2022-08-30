use super::*;

impl Logic<'_> {
    pub fn process_humans(&mut self) {
        let config = &self.model.assets.config;
        let mut rng = global_rng();
        for human in &mut self.model.humans {
            // Knock out timer
            if let Some(timer) = &mut human.knock_out_timer {
                *timer -= self.delta_time;
                if *timer <= Time::ZERO {
                    human.knock_out_timer = None;
                }
            }

            // Behaviour
            match &human.human_type {
                HumanType::Carrier {
                    holding_gun: Some(_),
                } => {
                    // Run around (panic)
                    let speed = config.human_run_speed;
                    let angle_delta = Rotation::new(
                        rng.gen_range(-config.human_turn_speed..=config.human_turn_speed)
                            * self.delta_time,
                    );
                    let rotation = Rotation::new(human.velocity.arg()) + angle_delta;
                    human.velocity = rotation.direction() * speed;
                }
                HumanType::Carrier { holding_gun: None } | HumanType::Pusher => {
                    // Walk around
                    let speed = config.human_walk_speed;
                    let angle_delta = Rotation::new(
                        rng.gen_range(-config.human_turn_speed..=config.human_turn_speed)
                            * self.delta_time,
                    );
                    let rotation = Rotation::new(human.velocity.arg()) + angle_delta;
                    human.velocity = rotation.direction() * speed;
                }
            }
        }
    }
}
