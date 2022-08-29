use super::*;

impl Model {
    pub fn new_player(&mut self) -> PlayerId {
        let id = self.id_gen.next_player();
        let player = Player {
            id,
            state: PlayerState::Spectator,
            score: 0,
        };
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
