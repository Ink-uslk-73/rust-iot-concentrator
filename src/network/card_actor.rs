//! src/network/card_actor.rs - CardActor (runact, P3.48).

use std::sync::Arc;
use runact::{Actor, ActorRef, Context};
use super::card::NetworkCard;

pub enum CardMsg {
    Up,
    Down,
    Stats,
}

pub struct CardActor {
    pub card: Arc<dyn NetworkCard>,
}

impl Actor for CardActor {
    type Message = CardMsg;

    async fn handle(&mut self, msg: CardMsg, _ctx: &mut Context<Self>) {
        match msg {
            CardMsg::Up => { let _ = self.card.up(); }
            CardMsg::Down => { let _ = self.card.down(); }
            CardMsg::Stats => { let _ = self.card.stats(); }
        }
    }
}

pub fn spawn_card_actor(card: Arc<dyn NetworkCard>) -> ActorRef<CardMsg> {
    ActorRef::spawn(CardActor { card })
}
