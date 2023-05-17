use geng::ui::*;

use super::*;

pub struct Joystick<'a> {
    is_captured: &'a mut Option<usize>,
    position: &'a mut Option<AABB<f64>>,
    stick_pos: &'a mut Vec2<f64>,
}

impl<'a> Joystick<'a> {
    pub fn new(cx: &'a Controller) -> Self {
        Self {
            is_captured: cx.get_state(),
            position: cx.get_state(),
            stick_pos: cx.get_state_with(|| Vec2::ZERO),
        }
    }

    pub fn get_stick_pos(&mut self) -> Vec2<f64> {
        *self.stick_pos
    }
}

impl<'a> Widget for Joystick<'a> {
    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints::default()
    }

    fn draw(&mut self, cx: &mut DrawContext) {
        *self.position = Some(cx.position);
        let aabb = cx.position.map(|x| x as f32);
        let radius = (aabb.width().min(aabb.height())) / 2.0;
        geng::draw_2d::Ellipse::circle(aabb.center(), radius, Rgba::GRAY).draw_2d(
            cx.geng,
            cx.framebuffer,
            &geng::PixelPerfectCamera,
        );
        geng::draw_2d::Ellipse::circle(
            aabb.center() + self.stick_pos.map(|x| x as f32) * radius,
            radius * 0.2,
            Rgba::BLACK,
        )
        .draw_2d(cx.geng, cx.framebuffer, &geng::PixelPerfectCamera);
    }

    fn handle_event(&mut self, event: &geng::Event) {
        let Some(aabb) = *self.position else { return };
        match event {
            geng::Event::TouchStart { touches } => {
                // Check whether the touch captures the joystick
                *self.is_captured = touches
                    .iter()
                    .position(|touch| aabb.contains(touch.position));
                info!("Joystick");
            }
            geng::Event::TouchMove { touches } => {
                if let Some(index) = *self.is_captured {
                    // Update stick position
                    let radius = (aabb.width().min(aabb.height())) / 2.0;
                    let position = (touches[index].position - aabb.center()) / radius;
                    *self.stick_pos = position.clamp_len(..=1.0);
                }
            }
            geng::Event::TouchEnd { .. } if self.is_captured.is_some() => {
                // Reset stick position
                *self.is_captured = None;
                *self.stick_pos = Vec2::ZERO;
            }
            _ => {}
        }
    }
}
