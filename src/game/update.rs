use super::*;

const CAMERA_INTERPOLATION: f32 = 0.5;

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        let model = self.model.get();
        let config = &model.assets.config;

        if let Some(player) = model.players.get(&self.player_id) {
            if let PlayerState::Gun { gun_id } = &player.state {
                if let Some(gun) = model.guns.get(gun_id) {
                    self.camera_target_position = gun.position;
                }
            }
        }

        self.camera.center.shift(
            self.camera
                .center
                .direction(&self.camera_target_position, config.arena_size)
                / Coord::new(CAMERA_INTERPOLATION)
                * delta_time,
            config.arena_size,
        );

        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_pos = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
            .map(Coord::new);

        self.model.send(Message::Aim { target: mouse_pos });
    }
}
