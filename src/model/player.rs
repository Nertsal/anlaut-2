use super::*;

impl Model {
    pub fn new_player(&mut self) -> PlayerId {
        let config = &self.assets.config;

        let id = self.id_gen.next_player();
        let player = Player {
            id,
            state: PlayerState::Spectator,
            score: 0,
        };

        if self.players.is_empty() {
            self.state = GameState::Finished {
                time_left: config.game_initial_delay,
                stats: GameStats { scores: default() },
            }
        }

        self.players.insert(player);
        id
    }

    pub fn drop_player(&mut self, player_id: PlayerId) {
        if let Some(player) = self.players.remove(&player_id) {
            if let PlayerState::Gun { gun_id } = &player.state {
                self.guns.remove(gun_id);
            }
        }
    }
}
