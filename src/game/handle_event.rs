use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_pos = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
            .map(Coord::new);

        match event {
            geng::Event::MouseDown {
                button: geng::MouseButton::Left,
                ..
            } => {
                self.model.send(Message::Shoot {
                    direction: vec2(1.0, 1.0).map(Coord::new),
                });
            }
            geng::Event::KeyDown { key } => match key {
                geng::Key::Num2 => {
                    self.model.send(Message::SpawnHuman {
                        position: mouse_pos,
                    });
                }
                geng::Key::Num3 => {
                    self.model.send(Message::SpawnGun {
                        position: mouse_pos,
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
