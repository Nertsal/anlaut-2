use super::*;

impl Game {
    pub fn draw_ui(&self, framebuffer: &mut ugli::Framebuffer) {
        let model = self.model.get();

        let player = model.players.get(&self.player_id);

        self.draw_online(model.players.len(), framebuffer);

        match &model.state {
            GameState::InProgress => {
                if let Some(player) = player {
                    self.draw_score(player.score, framebuffer);
                    if let PlayerState::Respawning { time_left } = player.state {
                        self.draw_respawn(time_left, framebuffer);
                    }
                }
            }
            GameState::Finished { time_left, stats } => {
                self.draw_finished(*time_left, stats, framebuffer)
            }
        }
    }

    fn draw_score(&self, score: Score, framebuffer: &mut ugli::Framebuffer) {
        draw_text(
            format!("Score: {}", score),
            Rgba::WHITE,
            0.02,
            vec2(0.5, 1.0),
            vec2(0.0, -0.05),
            vec2(0.5, 1.0),
            &self.geng,
            framebuffer,
        );
    }

    fn draw_online(&self, players: usize, framebuffer: &mut ugli::Framebuffer) {
        draw_text(
            format!("Players online: {}", players),
            Rgba::WHITE,
            0.015,
            vec2(1.0, 1.0),
            vec2(-0.05, -0.05),
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
            vec2(0.5, 0.5),
            vec2(0.0, 0.1),
            vec2(0.5, 0.0),
            &self.geng,
            framebuffer,
        );
    }

    fn draw_finished(
        &self,
        time_left: Time,
        stats: &GameStats,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        draw_text(
            "Game finished!",
            Rgba::WHITE,
            0.03,
            vec2(0.5, 1.0),
            vec2(0.0, -0.1),
            vec2(0.5, 1.0),
            &self.geng,
            framebuffer,
        );
        if let Some(score) = stats.scores.get(&self.player_id) {
            draw_text(
                format!("You scored {}", score),
                Rgba::WHITE,
                0.05,
                vec2(0.5, 0.5),
                vec2(0.0, 0.1),
                vec2(0.5, 0.0),
                &self.geng,
                framebuffer,
            );
            let mut scores: Vec<_> = stats.scores.iter().collect();
            scores.sort_by_key(|(_, score)| std::cmp::Reverse(*score));
            let place = scores
                .iter()
                .enumerate()
                .find(|(_, (id, _))| **id == self.player_id)
                .unwrap()
                .0
                + 1;
            draw_text(
                format!("Your place is {}", place),
                Rgba::WHITE,
                0.05,
                vec2(0.5, 0.5),
                vec2(0.0, 0.0),
                vec2(0.5, 1.0),
                &self.geng,
                framebuffer,
            );
        }
        draw_text(
            format!("Restarting in {:.0}s", time_left),
            Rgba::WHITE,
            0.03,
            vec2(0.5, 0.0),
            vec2(0.0, 0.1),
            vec2(0.5, 0.0),
            &self.geng,
            framebuffer,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_text(
    text: impl AsRef<str>,
    color: Rgba<f32>,
    font_size: f32,
    anchor: Vec2<f32>,
    offset: Vec2<f32>,
    alignment: Vec2<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
) {
    let screen = AABB::ZERO.extend_positive(framebuffer.size().map(|x| x as f32));
    let font = &**geng.default_font();
    let text = text.as_ref();
    let size_ref = screen.height().min(screen.width());

    let font_size = font_size * size_ref;
    let offset = offset * size_ref;
    let alignment = alignment - vec2(0.5, 0.5);
    let alignment = font
        .measure_bounding_box(text)
        .map(|measure| -measure.size() * alignment * font_size * 4.0) // I have no idea why 4.0
        .unwrap_or(Vec2::ZERO);
    let position = anchor * screen.size() + alignment + offset;

    draw_2d::Text::unit(font, text, color)
        .scale_uniform(font_size)
        .translate(position)
        .draw_2d(geng, framebuffer, &geng::PixelPerfectCamera);
}
