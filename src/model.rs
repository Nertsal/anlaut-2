use super::*;

mod collider;
mod gen;
mod guns;
mod id;
mod logic;
mod player;
mod position;
mod rotation;

pub use collider::*;
pub use id::*;
pub use position::*;
pub use rotation::*;

pub type Time = R32;
pub type Coord = R32;
pub type Score = u64;

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq)]
pub struct Model {
    id_gen: IdGen,
    #[diff = "eq"]
    pub state: GameState,
    pub assets: ServerAssets,
    pub players: Collection<Player>,
    pub humans: Collection<Human>,
    pub guns: Collection<Gun>,
    pub projectiles: Collection<Projectile>,
    pub blocks: Collection<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameStats {
    pub scores: HashMap<PlayerId, Score>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameState {
    InProgress { time_left: Time },
    Finished { time_left: Time, stats: GameStats },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerState {
    Spectator,
    Respawning { time_left: Time },
    Gun { gun_id: Id },
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Player {
    pub id: PlayerId,
    #[diff = "eq"]
    pub state: PlayerState,
    pub score: Score,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq)]
pub struct DeathInfo {
    pub killer: Option<PlayerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Human {
    pub id: Id,
    #[diff = "eq"]
    pub death: Option<DeathInfo>,
    pub position: Position,
    pub velocity: Vec2<Coord>,
    #[diff = "eq"]
    pub collider: Collider,
    pub holding_gun: Option<Id>,
    pub knock_out_timer: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Gun {
    pub id: Id,
    pub owner: Option<PlayerId>,
    #[diff = "eq"]
    pub death: Option<DeathInfo>,
    pub position: Position,
    pub rotation: Rotation,
    pub velocity: Vec2<Coord>,
    #[diff = "eq"]
    pub collider: Collider,
    pub attached_human: Option<Id>,
    pub aiming_at_host: bool,
    pub next_reload: Time,
    pub ammo: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Projectile {
    pub id: Id,
    pub caster: Option<PlayerId>,
    pub lifetime: Time,
    pub position: Position,
    pub velocity: Vec2<Coord>,
    #[diff = "eq"]
    pub collider: Collider,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Block {
    pub id: Id,
    pub position: Position,
    #[diff = "eq"]
    pub collider: Collider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Aim { target: Vec2<Coord> },
    Shoot { heavy: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Shoot {
        position: Position,
        direction: Vec2<Coord>,
    },
    ProjectileCollide {
        position: Position,
        velocity: Vec2<Coord>,
    },
}

impl Model {
    pub fn new(assets: ServerAssets) -> Self {
        let config = &assets.config;
        let mut model = Self {
            id_gen: IdGen::new(),
            state: GameState::InProgress {
                time_left: config.round_time,
            },
            players: default(),
            humans: default(),
            guns: default(),
            projectiles: default(),
            blocks: default(),
            assets,
        };
        model.generate_arena();
        model
    }
}

impl net::Model for Model {
    type PlayerId = PlayerId;
    type Message = Message;
    type Event = Event;
    const TICKS_PER_SECOND: f32 = TICKS_PER_SECOND;

    fn new_player(&mut self, _events: &mut Vec<Self::Event>) -> Self::PlayerId {
        self.new_player()
    }

    fn drop_player(&mut self, _events: &mut Vec<Self::Event>, player_id: &Self::PlayerId) {
        self.drop_player(*player_id)
    }

    fn handle_message(
        &mut self,
        events: &mut Vec<Self::Event>,
        player_id: &Self::PlayerId,
        message: Self::Message,
    ) {
        match message {
            Message::Aim { target } => {
                if let Some(player) = self.players.get(player_id) {
                    if let PlayerState::Gun { gun_id } = player.state {
                        let target = Position::from_world(target, self.assets.config.arena_size);
                        self.gun_aim(gun_id, target);
                    }
                }
            }
            Message::Shoot { heavy } => {
                if let Some(player) = self.players.get(player_id) {
                    if let PlayerState::Gun { gun_id } = player.state {
                        self.gun_shoot(gun_id, heavy, events);
                    }
                }
            }
        }
    }

    fn tick(&mut self, events: &mut Vec<Self::Event>) {
        let delta_time = Time::ONE / Time::new(Self::TICKS_PER_SECOND);
        self.update(delta_time, events);
    }
}
