use super::*;

use crate::model::{Event, Message, Model, PlayerId, Time};
use simple_net::Model as NetModel;

pub enum Connection {
    Local(Local),
    Remote(simple_net::Remote<Model>),
}

pub struct Local {
    player_id: PlayerId,
    model: RefCell<Model>,
    messages: RefCell<Vec<Message>>,
}

impl Connection {
    pub fn local(model: Model, player_id: PlayerId) -> Self {
        Self::Local(Local {
            player_id,
            model: RefCell::new(model),
            messages: default(),
        })
    }

    pub fn get(&self) -> Ref<Model> {
        match self {
            Connection::Local(local) => local.model.borrow(),
            Connection::Remote(remote) => remote.get(),
        }
    }

    pub fn update(&self, delta_time: Time) -> Vec<Event> {
        match self {
            Connection::Local(local) => {
                let player_id = local.player_id;
                let mut events = Vec::new();
                let mut model = local.model.borrow_mut();
                for message in std::mem::take(&mut *local.messages.borrow_mut()) {
                    model.handle_message(&mut events, &player_id, message);
                }
                model.update(delta_time, &mut events);
                events
            }
            Connection::Remote(remote) => remote.update(),
        }
    }

    pub fn send(&self, message: Message) {
        match self {
            Connection::Local(local) => {
                local.messages.borrow_mut().push(message);
            }
            Connection::Remote(remote) => remote.send(message),
        }
    }
}
