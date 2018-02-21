extern crate robots;

use robots::actors::{Actor, ActorCell, ActorContext, ActorRef};
use std::any::Any;

pub struct Resolver;

impl Actor for Resolver {
    fn receive(&self, message: Box<Any>, context: ActorCell) {
        if let Ok(message) = Box::<Any>::downcast::<String>(message) {
            let future = context.identify_actor(*message, "resolver_request".to_owned());
            context.forward_result_to_future::<Option<ActorRef>>(future, context.sender());
            context.stop(context.actor_ref());
        }
    }
}

impl Resolver {
    pub fn new(_dummy: ()) -> Resolver {
        Resolver
    }
}
