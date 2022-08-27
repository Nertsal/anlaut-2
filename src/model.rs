use super::*;

mod collider;
mod gen;
mod guns;
mod id;
mod logic;
mod position;
mod rotation;

pub use collider::*;
pub use id::*;
pub use position::*;
pub use rotation::*;

pub type Time = R32;
pub type Coord = R32;

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq)]
pub struct Model {
    id_gen: IdGen,
    pub assets: ServerAssets,
    pub players: Collection<Player>,
    pub humans: Collection<Human>,
    pub guns: Collection<Gun>,
    pub projectiles: Collection<Projectile>,
    pub blocks: Collection<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerState {
    Lobby,
    Gun { gun_id: Id },
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Player {
    pub id: PlayerId,
    #[diff = "eq"]
    pub state: PlayerState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Human {
    pub id: Id,
    pub is_alive: bool,
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

pub type Event = ();

impl Model {
    pub fn new(assets: ServerAssets) -> Self {
        let mut model = Self {
            id_gen: IdGen::new(),
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
        let gun_id = self.id_gen.next();
        let mut rng = global_rng();
        let gun = Gun {
            id: gun_id,
            position: Position::random(&mut rng, self.assets.config.arena_size),
            rotation: Rotation::ZERO,
            velocity: Vec2::ZERO,
            collider: Collider::Aabb {
                size: self.assets.config.gun_size,
            },
            attached_human: None,
            aiming_at_host: false,
            next_reload: Time::ZERO,
            ammo: 0,
        };
        self.guns.insert(gun);

        let id = self.id_gen.next_player();
        let player = Player {
            id,
            state: PlayerState::Gun { gun_id },
        };
        self.players.insert(player);
        id
    }

    fn drop_player(&mut self, _events: &mut Vec<Self::Event>, player_id: &Self::PlayerId) {
        if let Some(player) = self.players.remove(player_id) {
            if let PlayerState::Gun { gun_id } = &player.state {
                self.guns.remove(gun_id);
            }
        }
    }

    fn handle_message(
        &mut self,
        _events: &mut Vec<Self::Event>,
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
                        self.gun_shoot(gun_id, heavy);
                    }
                }
            }
        }
    }

    fn tick(&mut self, _events: &mut Vec<Self::Event>) {
        let delta_time = Time::ONE / Time::new(Self::TICKS_PER_SECOND);
        self.update(delta_time);
    }
}
