use super::*;

pub fn get_transform(
    position: Position,
    rotation: Rotation,
    world_size: Vec2<Coord>,
    camera: &CameraTorus2d,
) -> Mat3<Coord> {
    let position = camera.project(position, world_size);
    Mat3::translate(position) * Mat3::rotate(rotation.angle())
}

pub fn draw_collider(
    collider: &Collider,
    transform: Mat3<Coord>,
    color: Rgba<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &CameraTorus2d,
) {
    match collider {
        &Collider::Aabb { size } => {
            let aabb = AABB::ZERO.extend_symmetric(size / Coord::new(2.0));
            draw_quad_frame(
                aabb,
                transform,
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
