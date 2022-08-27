use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Collider {
    /// An Axis-Aligned Bounding Box
    Aabb { size: Vec2<Coord> },
}

#[derive(Debug, Clone)]
pub struct Collision {
    pub normal: Vec2<Coord>,
    pub penetration: Coord,
}

impl Collider {
    pub fn check(&self, other: &Self, delta_pos: Vec2<Coord>) -> bool {
        match (self, other) {
            (Collider::Aabb { size: size_a }, Collider::Aabb { size: size_b }) => {
                let a = AABB::ZERO.extend_symmetric(*size_a / Coord::new(2.0));
                let b = AABB::point(delta_pos).extend_symmetric(*size_b / Coord::new(2.0));
                a.x_min < b.x_max && a.x_max > b.x_min && a.y_min < b.y_max && a.y_max > b.y_min
            }
        }
    }

    pub fn collision(&self, other: &Self, delta_pos: Vec2<Coord>) -> Option<Collision> {
        match (self, other) {
            (Collider::Aabb { size: size_a }, Collider::Aabb { size: size_b }) => {
                let a = AABB::ZERO.extend_symmetric(*size_a / Coord::new(2.0));
                let b = AABB::point(delta_pos).extend_symmetric(*size_b / Coord::new(2.0));

                let dx_right = a.x_max - b.x_min;
                let dx_left = b.x_max - a.x_min;
                let dy_up = a.y_max - b.y_min;
                let dy_down = b.y_max - a.y_min;

                let (nx, px) = if dx_right < dx_left {
                    (Coord::ONE, dx_right)
                } else {
                    (-Coord::ONE, dx_left)
                };
                let (ny, py) = if dy_up < dy_down {
                    (Coord::ONE, dy_up)
                } else {
                    (-Coord::ONE, dy_down)
                };

                if px <= Coord::ZERO || py <= Coord::ZERO {
                    None
                } else if px < py {
                    Some(Collision {
                        normal: vec2(nx, Coord::ZERO),
                        penetration: px,
                    })
                } else {
                    Some(Collision {
                        normal: vec2(Coord::ZERO, ny),
                        penetration: py,
                    })
                }
            }
        }
    }
}
