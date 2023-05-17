use super::*;

use geng::ui::*;

mod joystick;

use joystick::*;

impl Game {
    pub fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        let control_ui = match self.control_mode {
            ControlMode::Touch => {
                let mut stick = Joystick::new(cx);
                let model = self.model.get();
                if let Some(gun_pos) = model
                    .players
                    .get(&self.player_id)
                    .and_then(|player| {
                        if let PlayerState::Gun { gun_id } = &player.state {
                            model.guns.get(gun_id)
                        } else {
                            None
                        }
                    })
                    .map(|gun| gun.position.to_world())
                {
                    let pos = stick.get_stick_pos();
                    self.model.send(Message::Aim {
                        target: gun_pos + pos.map(|x| Coord::new(x as f32)),
                    });
                }
                Box::new(geng::ui::stack![stick
                    .fixed_size(vec2(100.0, 100.0))
                    .align(vec2(0.0, 0.0))
                    .uniform_padding(50.0)]) as Box<dyn geng::ui::Widget>
            }
            ControlMode::Mouse => Box::new(geng::ui::Void),
        };
        control_ui
    }
}
