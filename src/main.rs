extern crate env_logger;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate robots;
extern crate router;
extern crate staticfile;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::sync::Arc;
use persistent::Read;
use iron::typemap::Key;
use robots::actors::{ActorRef, ActorSystem, Props};
use actorimpl::dragon::{Dragon, DragonCommands, DragonEvents};
use actorimpl::common::Resolver;

pub mod gpioaccess;
pub mod actorimpl;

#[derive(Copy, Clone)]
pub struct Sys;
impl Key for Sys {
    type Value = ActorSystem;
}

fn main() {
    match env_logger::init() {
        Ok(()) => {}
        _ => {}
    };
    let dragon_actor_system = ActorSystem::new("dragon".to_owned());
    dragon_actor_system.spawn_threads(3);

    dragon_actor_system.actor_of(Props::new(Arc::new(Dragon::new), ()), "gorynich".to_owned());
    // the router (for RESTfull actions)
    let mut router = Router::new();
    router.post("/wings", move_wings, "wings");
    router.post("/mouth", open_mouth, "mouth");
    router.post("/eyes", blink_eyes, "eyes");

    fn handle_command(
        req: &mut Request,
        dragon_command: DragonCommands,
        dragon_event: DragonEvents,
    ) -> IronResult<Response> {
        let arcsys = req.get::<Read<Sys>>().unwrap();
        let sys = arcsys.as_ref();
        let props = Props::new(Arc::new(Resolver::new), ());
        let answerer = sys.actor_of(props.clone(), "answerer".to_owned());
        let dragon = sys.ask(answerer, "/user/gorynich".to_owned(), "future".to_owned());
        let dragon: Option<ActorRef> = sys.extract_result(dragon);
        match dragon {
            None => Ok(Response::with((status::Gone, "Dragon not found"))),
            Some(dragonunwrapped) => {
                let future = sys.ask(dragonunwrapped, dragon_command, "request".to_owned());
                let event: DragonEvents = sys.extract_result(future);
                if event == dragon_event {
                    Ok(Response::with((status::Ok, dragon_event.to_string())))
                } else {
                    Ok(Response::with((status::MethodNotAllowed, "Unknown event!")))
                }
            }
        }
    }

    fn move_wings(req: &mut Request) -> IronResult<Response> {
        handle_command(req, DragonCommands::MoveWings, DragonEvents::WingsMoved)
    }

    fn open_mouth(req: &mut Request) -> IronResult<Response> {
        handle_command(req, DragonCommands::OpenMouth, DragonEvents::MouthOpened)
    }

    fn blink_eyes(req: &mut Request) -> IronResult<Response> {
        handle_command(req, DragonCommands::BlinkEyes, DragonEvents::EyesBlinked)
    }

    let mut chain = Chain::new(router);
    chain.link(Read::<Sys>::both(dragon_actor_system));
    // the mounter for static files
    let mut mount = Mount::new();
    mount
        .mount("/", Static::new(Path::new("irondragonfe/dist/")))
        .mount("/api/", chain);
    Iron::new(mount).http("0.0.0.0:8080").unwrap();
}

mod test;

mod tests;
