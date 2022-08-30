use super::*;

impl Render {
    pub fn draw_ui(&self, model: &Model, player_id: PlayerId, framebuffer: &mut ugli::Framebuffer) {
        self.draw_online(model.players.len(), framebuffer);

        match &model.state {
            GameState::InProgress { time_left } => {
                self.draw_in_progress(model, player_id, *time_left, framebuffer);
            }
            GameState::Finished { time_left, stats } => {
                self.draw_finished(player_id, *time_left, stats, framebuffer)
            }
        }
    }

    fn draw_in_progress(
        &self,
        model: &Model,
        player_id: PlayerId,
        time_left: Time,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let player = model.players.get(&player_id);

        if let Some(player) = player {
            let mut position = ScreenPosition {
                anchor: vec2(0.5, 1.0),
                offset: vec2(0.0, -0.05),
            };
            let score = match player.state {
                PlayerState::Spectator => {
                    draw_text(
                        "Spectating",
                        Rgba::WHITE,
                        0.04,
                        ScreenPosition {
                            anchor: vec2(0.5, 1.0),
                            offset: vec2(0.0, -0.02),
                        },
                        vec2(0.5, 1.0),
                        &self.geng,
                        framebuffer,
                    );
                    position.offset.y -= 0.1;
                    0
                }
                PlayerState::Respawning { time_left } => {
                    self.draw_respawn(time_left, framebuffer);
                    player.score
                }
                PlayerState::Gun { .. } => player.score,
            };
            draw_text(
                format!("Score: {}", score),
                Rgba::WHITE,
                0.02,
                position,
                vec2(0.5, 1.0),
                &self.geng,
                framebuffer,
            );
        }

        draw_text(
            format!("Time remaining: {:.0}s", time_left),
            Rgba::WHITE,
            0.02,
            ScreenPosition {
                anchor: vec2(0.5, 0.0),
                offset: vec2(0.0, 0.05),
            },
            vec2(0.5, 0.0),
            &self.geng,
            framebuffer,
        );
    }

    fn draw_online(&self, players: usize, framebuffer: &mut ugli::Framebuffer) {
        draw_text(
            format!("Players online: {}", players),
            Rgba::WHITE,
            0.015,
            ScreenPosition {
                anchor: vec2(1.0, 1.0),
                offset: vec2(-0.05, -0.05),
            },
            vec2(1.0, 1.0),
            &self.geng,
            framebuffer,
        );
    }

    fn draw_respawn(&self, time_left: Time, framebuffer: &mut ugli::Framebuffer) {
        draw_text(
            format!("Respawning in {:.0}s", time_left),
            Rgba::WHITE,
            0.05,
            ScreenPosition {
                anchor: vec2(0.5, 0.5),
                offset: vec2(0.0, 0.1),
            },
            vec2(0.5, 0.0),
            &self.geng,
            framebuffer,
        );
    }

    fn draw_finished(
        &self,
        player_id: PlayerId,
        time_left: Time,
        stats: &GameStats,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        draw_text(
            "Game finished!",
            Rgba::WHITE,
            0.03,
            ScreenPosition {
                anchor: vec2(0.5, 1.0),
                offset: vec2(0.0, -0.1),
            },
            vec2(0.5, 1.0),
            &self.geng,
            framebuffer,
        );
        if let Some(score) = stats.scores.get(&player_id) {
            draw_text(
                format!("You scored {}", score),
                Rgba::WHITE,
                0.05,
                ScreenPosition {
                    anchor: vec2(0.5, 0.5),
                    offset: vec2(0.0, 0.1),
                },
                vec2(0.5, 0.0),
                &self.geng,
                framebuffer,
            );
            let mut sorted = Vec::<(Vec<PlayerId>, Score)>::new();
            for (&player, &score) in &stats.scores {
                use std::cmp::Reverse;
                match sorted.binary_search_by_key(&Reverse(score), |(_, s)| Reverse(*s)) {
                    Ok(idx) => sorted[idx].0.push(player),
                    Err(idx) => sorted.insert(idx, (vec![player], score)),
                }
            }
            let scores = sorted;
            let place = scores
                .iter()
                .enumerate()
                .find(|(_, (ids, _))| ids.contains(&player_id))
                .unwrap()
                .0
                + 1;
            draw_text(
                format!("Your place is {}", place),
                Rgba::WHITE,
                0.05,
                ScreenPosition {
                    anchor: vec2(0.5, 0.5),
                    offset: vec2(0.0, 0.0),
                },
                vec2(0.5, 1.0),
                &self.geng,
                framebuffer,
            );
        }
        draw_text(
            format!("Restarting in {:.0}s", time_left),
            Rgba::WHITE,
            0.03,
            ScreenPosition {
                anchor: vec2(0.5, 0.0),
                offset: vec2(0.0, 0.1),
            },
            vec2(0.5, 0.0),
            &self.geng,
            framebuffer,
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct ScreenPosition {
    pub anchor: Vec2<f32>,
    pub offset: Vec2<f32>,
}

impl ScreenPosition {
    pub fn to_screen(self, screen_size: Vec2<f32>) -> Vec2<f32> {
        let size_ref = screen_size.x.min(screen_size.y);
        self.anchor * screen_size + self.offset * size_ref
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_text(
    text: impl AsRef<str>,
    color: Rgba<f32>,
    font_size: f32,
    position: ScreenPosition,
    alignment: Vec2<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    let screen = AABB::ZERO.extend_positive(framebuffer.size().map(|x| x as f32));
    let font = &**geng.default_font();
    let text = text.as_ref();
    let size_ref = screen.height().min(screen.width());

    let font_size = font_size * size_ref;
    let alignment = alignment - vec2(0.5, 0.5);
    let alignment = font
        .measure_bounding_box(text)
        .map(|measure| -measure.size() * alignment * font_size * 4.0) // I have no idea why 4.0
        .unwrap_or(Vec2::ZERO);
    let position = position.to_screen(screen.size()) + alignment;

    draw_2d::Text::unit(font, text, color)
        .scale_uniform(font_size)
        .translate(position)
        .draw_2d(geng, framebuffer, &geng::PixelPerfectCamera);
}
