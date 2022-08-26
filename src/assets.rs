use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Diff)]
pub struct ServerAssets {
    pub config: Config,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Diff)]
pub struct Config {
    pub arena_size: Vec2<f32>,
    pub human_knockout_time: f32,
    pub human_walk_speed: f32,
    pub human_run_speed: f32,
    pub human_turn_speed: f32,
    pub gun_size: Vec2<f32>,
    pub gun_shoot_speed: f32,
    pub gun_recoil_speed: f32,
    pub gun_friction: f32,
    pub gun_orbit_radius: f32,
    pub projectile_lifetime: f32,
}

#[derive(geng::Assets)]
pub struct Assets {}

impl Assets {
    pub async fn process(&mut self, _geng: &Geng) {}
}
