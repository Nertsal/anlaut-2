use super::*;

const INTERPOLATION_TIME: f32 = 0.05;

#[derive(Debug, Clone)]
pub struct Interpolation {
    position: Position,
    velocity: Vec2<Coord>,
    current_time: Time,
}

impl Interpolation {
    pub fn new(position: Position, velocity: Vec2<Coord>) -> Self {
        Self {
            position,
            velocity,
            current_time: Time::ZERO,
        }
    }

    pub fn get_interpolated(&self) -> Position {
        self.position
    }

    pub fn update(
        &mut self,
        delta_time: Time,
        target_position: Position,
        target_velocity: Vec2<Coord>,
        world_size: Vec2<Coord>,
    ) {
        let start_time = self.current_time;
        let end_time = start_time + Time::new(INTERPOLATION_TIME);
        self.current_time += delta_time;

        let t = (self.current_time - start_time) / (end_time - start_time); // Interpolation parameter
        let t2 = t * t;
        let t3 = t2 * t;

        let one = Coord::ONE;
        let two = Coord::new(2.0);
        let three = Coord::new(3.0);
        let four = Coord::new(4.0);
        let six = Coord::new(6.0);

        let p0 = self.position.to_world();
        let p1 = p0 + self.position.direction(&target_position, world_size);
        let v0 = self.velocity;
        let v1 = target_velocity;

        // Cubic Hermite spline interpolation on a unit interval
        let position = p0 * (two * t3 - three * t2 + one)
            + v0 * (t3 - two * t2 + t) * (end_time - start_time)
            + p1 * (-two * t3 + three * t2)
            + v1 * (t3 - t2) * (end_time - start_time);
        let velocity = (p0 * (six * t2 - six * t)
            + v0 * (three * t2 - four * t + one) * (end_time - start_time)
            + p1 * (-six * t2 + six * t)
            + v1 * (three * t2 - two * t) * (end_time - start_time))
            * one
            / (end_time - start_time);

        self.position = Position::from_world(position, world_size);
        self.velocity = velocity;
    }
}
