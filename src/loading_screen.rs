use geng::{prelude::*, Draw2d};

pub struct LoadingScreen {
    geng: Geng,
}

impl LoadingScreen {
    pub fn new(geng: &Geng) -> Self {
        Self { geng: geng.clone() }
    }
}

impl geng::ProgressScreen for LoadingScreen {}

impl geng::State for LoadingScreen {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::WHITE), None);

        draw_2d::Text::unit(
            &**self.geng.default_font(),
            "Loading assets...",
            Rgba::BLACK,
        )
        .scale_uniform(40.0)
        .translate(framebuffer_size.map(|x| x as f32) / 2.0)
        .draw_2d(&self.geng, framebuffer, &geng::PixelPerfectCamera);
    }
}
