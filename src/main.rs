extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate robots;
extern crate sysfs_gpio;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::any::Any;
use robots::actors::{ActorSystem,Actor,ActorCell,ActorContext,Props};
use sysfs_gpio::{Direction, Pin};

fn main() {
	// the router (for RESTfull actions)
	let mut router = Router::new();
	
	router.get("/wings", move_wings,"wings");
	router.get("/mouth", open_mouth,"mouth");
	router.get("/eyes", blink_eyes,"eyes");

	fn move_wings(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Wings moved!")))
	}

	fn open_mouth(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Mouth opened!")))
	}

	fn blink_eyes(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Eyes blinked!")))
	}

	let dragon_actor_system = ActorSystem::new("dragon".to_owned());
	dragon_actor_system.spawn_threads(3);
	// the mounter for static files
	let mut mount = Mount::new();
	mount
		.mount("/",Static::new(Path::new("static")))
		.mount("/api/",router);
	Iron::new(mount).http("0.0.0.0:3000").unwrap();
}

#[derive(Copy, Clone, PartialEq)]
enum DragonCommands {
	MoveWings,
	OpenMouth,
	BlinkEyes,
}

struct Dragon;

impl Actor for Dragon {
	fn receive(&self, _message: Box<Any>, _context: ActorCell){
		if let Ok(_message) = Box::<Any>::downcast::<DragonCommands>(_message){
			match *_message {
				DragonCommands::MoveWings => println!("Moving Wings"),
				DragonCommands::OpenMouth => println!("Opening Mouth"),
				DragonCommands::BlinkEyes => println!("Blinking Eyes")
			}
		} else {
			println!("Gorynich does not understand!");
		}
	}
}

impl Dragon {
	fn new(_: ()) -> Dragon {
		Dragon
	}
}

struct Wings;

impl Actor for Wings {
    fn receive(&self, _message: Box<Any>, _context: ActorCell) {}
}

impl Wings {
    fn new(_: ()) -> Wings {
        Wings
    }
}

mod test; 
