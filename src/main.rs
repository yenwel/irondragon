extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate robots;
extern crate persistent;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::any::Any;
use std::sync::Arc;
use robots::actors::{ActorSystem,Actor,ActorCell,ActorContext,Props,ActorRef};
use persistent::Read;
use iron::typemap::Key;

#[derive(Copy, Clone)]
pub struct Sys;
impl Key for Sys { type Value = ActorSystem; }

#[derive(Clone, Copy)]
pub struct DragonActor;
impl Key for DragonActor { type Value = ActorRef; }

struct Resolver;

impl Actor for Resolver {
    fn receive(&self, message: Box<Any>, context: ActorCell) {
        if let Ok(message) = Box::<Any>::downcast::<String>(message) {
            let future = context.identify_actor(*message, "resolver_request".to_owned());
            context.forward_result_to_future::<Option<ActorRef>>(future, context.sender());
        }
    }
}

impl Resolver {
    fn new(_dummy: ()) -> Resolver {
        Resolver
    }
}


fn main() {
		
	let dragon_actor_system  = ActorSystem::new("dragon".to_owned());
	dragon_actor_system.spawn_threads(3);

	let props = Props::new(Arc::new(Dragon::new),());
	dragon_actor_system.actor_of(props, "gorynich".to_owned());
	
    // the router (for RESTfull actions)
	let mut router = Router::new();
	
	router.get("/wings", move_wings,"wings");
	router.get("/mouth", open_mouth,"mouth");
	router.get("/eyes", blink_eyes,"eyes"); 

	fn handle_command(req: &mut Request, dragon_command: DragonCommands, dragon_event: DragonEvents ) ->IronResult<Response> {
		let arcsys = req.get::<Read<Sys>>().unwrap();
    	let sys = arcsys.as_ref();
		let props = Props::new(Arc::new(Resolver::new), ());
    	let answerer = sys.actor_of(props.clone(), "answerer".to_owned());
        let dragon = sys.ask(answerer, "/user/gorynich".to_owned(), "future".to_owned());
    	let dragon: Option<ActorRef> = sys.extract_result(dragon);
		match dragon {
			None => {
				Ok(Response::with((status::Ok, "Dragon not found")))
			}
			Some(dragonunwrapped) => {
				let future = sys.ask(dragonunwrapped, dragon_command, "request".to_owned());
	    		let event: DragonEvents = sys.extract_result(future);
				if event == dragon_event {
					Ok(Response::with((status::Ok, "Event happened!")))
				} else
				{
					Ok(Response::with((status::Ok, "Unknown event!")))
				}				
			}
		}
	}

	fn move_wings(req:&mut Request)  -> IronResult<Response>  {
		handle_command(req,DragonCommands::MoveWings,DragonEvents::WingsMoved)
	}

	fn open_mouth(req: &mut Request) -> IronResult<Response> {		
		handle_command(req,DragonCommands::OpenMouth,DragonEvents::MouthOpened)
	}

	fn blink_eyes(req: &mut Request) -> IronResult<Response> {		
		handle_command(req,DragonCommands::BlinkEyes,DragonEvents::EyesBlinked)
	}

	let mut chain = Chain::new(router);
	chain.link(Read::<Sys>::both(dragon_actor_system));
	
	// the mounter for static files
	let mut mount = Mount::new();
	mount
		.mount("/",Static::new(Path::new("static")))
		.mount("/api/",chain);
	Iron::new(mount).http("0.0.0.0:3000").unwrap();
}

#[derive(Copy, Clone, PartialEq)]
enum DragonCommands {
	MoveWings,
	OpenMouth,
	BlinkEyes,
}

#[derive(Copy, Clone, PartialEq)]
enum DragonEvents {
	WingsMoved,
	MouthOpened,
	EyesBlinked,
}


struct Dragon;

impl Actor for Dragon {
	fn receive(&self, _message: Box<Any>, _context: ActorCell){
		if let Ok(_message) = Box::<Any>::downcast::<DragonCommands>(_message){
			match *_message {
				DragonCommands::MoveWings => {
						println!("Moving Wings");
						_context.complete(_context.sender(),DragonEvents::WingsMoved);
						},
				DragonCommands::OpenMouth => {
						println!("Opening Mouth");					
						_context.complete(_context.sender(),DragonEvents::MouthOpened);
						},
				DragonCommands::BlinkEyes => {
						println!("Blinking Eyes");					
						_context.complete(_context.sender(),DragonEvents::EyesBlinked);
						}
			}
		} else {
			println!("Gorynich does not understand!");
		}
	}
}

impl Dragon {
	fn new(_dummy: ()) -> Dragon {
		Dragon
	}
}

#[cfg(linux)]
mod gpioaccess{

	extern crate sysfs_gpio;
	use sysfs_gpio::{Direction, Pin};
}

/*struct Wings;

impl Actor for Wings {
    fn receive(&self, _message: Box<Any>, _context: ActorCell) {}
}

impl Wings {
    fn new(_: ()) -> Wings {
        Wings
    }
}*/

mod test; 
