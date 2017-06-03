extern crate robots;

use std::fmt::{Debug, Display,Formatter,Result};
use std::sync::Arc;
use std::any::Any;
use robots::actors::{Actor,ActorCell,Props,ActorRef,ActorPath,ActorContext};
use super::rpi::{PinActor,PwmActor,PwmCommands,PinCommands};

#[derive(Copy, Clone, PartialEq,Debug)]
pub enum DragonCommands {
	MoveWings,
	OpenMouth,
	BlinkEyes,
}

#[derive(Copy, Clone, PartialEq,Debug)]
pub enum DragonEvents {
	WingsMoved,
	MouthOpened,
	EyesBlinked,
}

impl Display for DragonCommands {
	fn fmt(&self, f: &mut Formatter) -> Result {
		Debug::fmt(self, f)
	}
}

impl Display for DragonEvents {
	fn fmt(&self, f: &mut Formatter) -> Result {
		Debug::fmt(self, f)
	}
}

pub struct Dragon;

impl Actor for Dragon {

	fn pre_start(&self, context: ActorCell) {
		let wings = context.actor_of(Props::new(Arc::new(Wings::new),()), "wings".to_owned()).unwrap();
		let mouth = context.actor_of(Props::new(Arc::new(Mouth::new),()), "mouth".to_owned()).unwrap();
		let eyes = context.actor_of(Props::new(Arc::new(Eyes::new),()), "eyes".to_owned()).unwrap();
		context.tell(wings,LimbCommands::Init(10));
		context.tell(mouth,LimbCommands::Init(10));
		context.tell(eyes,LimbCommands::Init(10));
	}
	fn receive(&self, _message: Box<Any>, _context: ActorCell){
		if let Ok(_message) = Box::<Any>::downcast::<DragonCommands>(_message){
			match *_message {
				DragonCommands::MoveWings => {
						let wings: ActorRef = _context.children().get(&ActorPath::new_local("/user/gorynich/wings".to_owned())).cloned().unwrap();
						_context.tell(wings,LimbCommands::Aggitate);
						_context.complete(_context.sender(),DragonEvents::WingsMoved);
						},
				DragonCommands::OpenMouth => {
						let mouth: ActorRef = _context.children().get(&ActorPath::new_local("/user/gorynich/mouth".to_owned())).cloned().unwrap();
						_context.tell(mouth,LimbCommands::Aggitate);
						_context.complete(_context.sender(),DragonEvents::MouthOpened);
						},
				DragonCommands::BlinkEyes => {
						let eyes: ActorRef = _context.children().get(&ActorPath::new_local("/user/gorynich/eyes".to_owned())).cloned().unwrap();
						_context.tell(eyes,LimbCommands::Aggitate);
						_context.complete(_context.sender(),DragonEvents::EyesBlinked);
						}
			}
		} else {
			println!("Gorynich does not understand!");
		}
	}
}

impl Dragon {
	pub fn new(_: ()) -> Dragon {
		Dragon
	}
}

#[derive(Copy, Clone, PartialEq,Debug)]
enum LimbCommands {
	Init(u64),
	Aggitate,
	Reset,
}

#[derive(Copy, Clone, PartialEq,Debug)]
enum LimbEvents {
	Aggitated,
	NotAggitated(u64),
}

impl Display for LimbCommands {
	fn fmt(&self, f: &mut Formatter) -> Result {
		Debug::fmt(self, f)
	}
}

impl Display for LimbEvents {
	fn fmt(&self, f: &mut Formatter) -> Result {
		Debug::fmt(self, f)
	}
}

struct Wings;

impl Actor for Wings {
	fn pre_start(&self, context: ActorCell) {
		   context.actor_of(Props::new(Arc::new(PwmActor::new), 0), "pwm18".to_owned()).unwrap();
	}
	fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		if let Ok(_message) = Box::<Any>::downcast::<LimbCommands>(_message){
			match *_message {
				LimbCommands::Init(max) => { println!("Initializing with maximum {}",max); },
				LimbCommands::Aggitate => {
					println!("Moving Wings");
					let pwm18 : ActorRef = _context.children().get(&ActorPath::new_local("/user/gorynich/wings/pwm18".to_owned())).cloned().unwrap();
					_context.tell(pwm18,PwmCommands::MoveToDegree(90));
				},
				LimbCommands::Reset => { println!("Received reset"); }
			}
		} else {
			println!("Gorynich does not understand!");
		}
	}
}

impl Wings {
	fn new(_: ()) -> Wings {
		Wings
	}
}

struct Mouth;

impl Actor for Mouth {
	fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		if let Ok(_message) = Box::<Any>::downcast::<LimbCommands>(_message){
			match *_message {
				LimbCommands::Init(max) => { println!("Initializing with maximum {}",max); },
				LimbCommands::Aggitate => {
					println!("Opening Mouth");
				},
				LimbCommands::Reset => { println!("Received reset"); }
			}
		} else {
			println!("Gorynich does not understand!");
		}
	}
}

impl Mouth {
	fn new(_: ()) -> Mouth {
		Mouth
	}
}

struct Eyes;

impl Actor for Eyes {

	fn pre_start(&self, context: ActorCell) {
		   context.actor_of(Props::new(Arc::new(PinActor::new), 22), "pin22".to_owned()).unwrap();
	}
	
	fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		if let Ok(_message) = Box::<Any>::downcast::<LimbCommands>(_message){
			match *_message {
				LimbCommands::Init(max) => { println!("Initializing with maximum {}",max); },
				LimbCommands::Aggitate => {
					println!("Blinking Eyes");
					let pin22 : ActorRef = _context.children().get(&ActorPath::new_local("/user/gorynich/eyes/pin22".to_owned())).cloned().unwrap();
					_context.tell(pin22,PinCommands::Blink(20));
				},
				LimbCommands::Reset => { println!("Received reset"); }
			}
		} else {
			println!("Gorynich does not understand!");
		}
	}
}

impl Eyes {
	fn new(_: ()) -> Eyes {
		Eyes
	}
}