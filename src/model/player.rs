use super::*;

impl Model {
    pub fn new_player(&mut self) -> PlayerId {
        let config = &self.assets.config;

        if self.players.is_empty() {
            for (color, free) in &mut self.colors {
                if !*free {
                    error!(
                        "Available colors got invalidated, specifically: {:?}",
                        color
                    );
                    *free = true;
                }
            }
            self.state = GameState::Finished {
                time_left: config.game_initial_delay,
                stats: GameStats { scores: default() },
            }
        }

        let id = self.id_gen.next_player();
        let player = Player {
            id,
            state: PlayerState::Spectator,
            score: 0,
            color: self.take_color(),
        };

        self.players.insert(player);
        id
    }

    pub fn drop_player(&mut self, player_id: PlayerId) {
        if let Some(player) = self.players.remove(&player_id) {
            if let PlayerState::Gun { gun_id } = &player.state {
                self.guns.remove(gun_id);
            }
            if let Some((_, free)) = self
                .colors
                .iter_mut()
                .find(|(color, _)| *color == player.color)
            {
                *free = true;
            }
        }
    }

    fn take_color(&mut self) -> Rgba<f32> {
        let mut rng = global_rng();
        if let Some((color, free)) = self
            .colors
            .iter_mut()
            .filter(|(_, free)| *free)
            .choose(&mut rng)
        {
            *free = false;
            *color
        } else {
            Rgba::BLUE
        }
    }
}
