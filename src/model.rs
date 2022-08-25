use super::*;

mod id;
mod update;

pub use id::*;

pub type Time = R32;

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq)]
pub struct Model {
    id_gen: IdGen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {}

pub type Event = ();

impl Model {
    pub fn new() -> Self {
        Self {
            id_gen: IdGen::new(),
        }
    }
}

impl simple_net::Model for Model {
    type PlayerId = Id;
    type Message = Message;
    type Event = Event;
    const TICKS_PER_SECOND: f32 = TICKS_PER_SECOND;

    fn new_player(&mut self, _events: &mut Vec<Self::Event>) -> Self::PlayerId {
        let id = self.id_gen.next();
        id
    }

    fn drop_player(&mut self, _events: &mut Vec<Self::Event>, player_id: &Self::PlayerId) {
        // TODO
    }

    fn handle_message(
        &mut self,
        _events: &mut Vec<Self::Event>,
        player_id: &Self::PlayerId,
        message: Self::Message,
    ) {
        // TODO
    }

    fn tick(&mut self, _events: &mut Vec<Self::Event>) {
        let delta_time = Time::ONE / Time::new(Self::TICKS_PER_SECOND);
        self.update(delta_time);
    }
}
