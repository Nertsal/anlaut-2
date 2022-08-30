use super::*;

pub struct Shake {
    offset: Vec2<Coord>,
    velocity: Vec2<Coord>,
    amplitude: Coord,
    speed: Coord,
    time: Time,
}

impl Shake {
    pub fn new() -> Self {
        Self {
            offset: Vec2::ZERO,
            velocity: Vec2::ZERO,
            amplitude: Coord::ZERO,
            speed: Coord::ZERO,
            time: Time::ZERO,
        }
    }

    pub fn start(&mut self, amplitude: Coord, speed: Coord, time: Time) {
        self.amplitude = amplitude;
        self.speed = speed;
        self.time = time;
        self.offset = Vec2::ZERO;

        let mut rng = global_rng();
        let angle = r32(rng.gen_range(0.0..=2.0 * f32::PI));
        let (sin, cos) = angle.sin_cos();
        self.velocity = vec2(cos, sin) * self.speed;
    }

    pub fn offset(&self) -> Vec2<Coord> {
        self.offset
    }

    pub fn update(&mut self, delta_time: Time) {
        if self.time <= Time::ZERO {
            self.offset -= self.offset.clamp_len(..=self.speed);
            return;
        }

        self.time -= delta_time;
        self.offset += self.velocity * delta_time;
        let len = self.offset.len();
        if len >= self.amplitude {
            // Clamp and bounce
            self.offset = self.offset / len * self.amplitude;
            let angle = r32(global_rng().gen_range(-0.1..=0.1));
            self.velocity = -self.velocity.rotate(angle);
        }
    }
}

impl Default for Shake {
    fn default() -> Self {
        Self::new()
    }
}
