use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let model = self.model.get();

        // Arena boundary
        draw_quad_frame(
            model.arena_bounds,
            Mat3::identity(),
            Coord::new(0.5),
            Rgba::GRAY,
            &self.geng,
            framebuffer,
            &self.camera,
        );

        for human in &model.humans {
            draw_collider(
                human.position,
                Rotation::ZERO,
                &human.collider,
                Rgba::GREEN,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for gun in &model.guns {
            draw_collider(
                gun.position,
                gun.rotation,
                &gun.collider,
                Rgba::BLUE,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for projectile in &model.projectiles {
            draw_collider(
                projectile.position,
                Rotation::ZERO,
                &projectile.collider,
                Rgba::RED,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
    }
}

pub fn draw_collider(
    position: Position,
    rotation: Rotation,
    collider: &Collider,
    color: Rgba<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    match collider {
        &Collider::Aabb { size } => {
            let aabb = AABB::ZERO.extend_symmetric(size / Coord::new(2.0));
            draw_quad_frame(
                aabb,
                Mat3::translate(position) * Mat3::rotate(rotation.angle()),
                Coord::new(0.1),
                color,
                geng,
                framebuffer,
                camera,
            );
        }
    }
}

pub fn draw_quad_frame(
    aabb: AABB<Coord>,
    transform: Mat3<Coord>,
    width: Coord,
    color: Rgba<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    let left_mid = vec2(aabb.x_min, aabb.center().y);
    let points = [
        left_mid,
        aabb.top_left(),
        aabb.top_right(),
        aabb.bottom_right(),
        aabb.bottom_left(),
        left_mid,
    ]
    .into_iter()
    .map(|point| {
        let point = transform * point.extend(Coord::ONE);
        (point.xy() / point.z).map(|x| x.as_f32())
    })
    .collect();
    let chain = Chain::new(points);
    draw_2d::Chain::new(chain, width.as_f32(), color, 3).draw_2d(geng, framebuffer, camera);
}
