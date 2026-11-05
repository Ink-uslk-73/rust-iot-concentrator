//! src/network/card_actor.rs - CardActor (runact, P3.48).

use std::sync::Arc;
use runact::{Actor, ActorContext, ActorError, ActorId, Runtime};
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

    fn handle(&mut self, msg: CardMsg, _ctx: &mut ActorContext) -> Result<(), ActorError> {
        match msg {
            CardMsg::Up => { let _ = self.card.up(); }
            CardMsg::Down => { let _ = self.card.down(); }
            CardMsg::Stats => { let _ = self.card.stats(); }
        }
        Ok(())
    }
}

pub fn spawn_card_actor(runtime: &mut Runtime, card: Arc<dyn NetworkCard>) -> Result<ActorId, runact::RuntimeError> {
    runtime.spawn(CardActor { card })
}