use super::*;

impl Game {
    pub fn draw_ui(&self, framebuffer: &mut ugli::Framebuffer) {
        let model = self.model.get();

        match &model.state {
            GameState::InProgress => {}
            GameState::Finished { time_left } => self.draw_finished(*time_left, framebuffer),
        }
    }

    fn draw_finished(&self, time_left: Time, framebuffer: &mut ugli::Framebuffer) {
        let screen = AABB::ZERO.extend_positive(framebuffer.size().map(|x| x as f32));
        let font = &**self.geng.default_font();
        draw_2d::Text::unit(
            font,
            format!("Finished!\nRestarting in {:.0}s", time_left),
            Rgba::WHITE,
        )
        .scale_uniform(50.0)
        .translate(screen.center() + vec2(0.0, 100.0))
        .draw_2d(&self.geng, framebuffer, &geng::PixelPerfectCamera);
    }
}
