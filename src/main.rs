extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate robots;
extern crate persistent;
extern crate env_logger;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::any::Any;
use std::sync::{Arc,Mutex};
use std::thread;
use robots::actors::{ActorSystem,Actor,ActorCell,ActorContext,Props,ActorRef,ActorPath};
use persistent::Read;
use iron::typemap::Key;
use std::fmt::{Debug, Display,Formatter,Result};
use gpioaccess::PinProxy;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectionProxied {
    In,
    Out,
    High,
    Low,
}


#[derive(Debug)]
pub enum ProxyError {
	MonErreur,
}

impl ::std::error::Error for ProxyError {
	fn description(&self) -> &str {
		"an error"
	}
}

impl Display for ProxyError {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "an error")
	}
}

pub type ProxyResult<T> = ::std::result::Result<T, ProxyError>;

//depend on abstractions not concretions lulz
trait PinProxyContract {

	fn new(pin_num: u64) -> Self;
	//FIXME :figure out mapping overloaded Result from gpio library
	fn export(&self) -> ProxyResult<()>;

	fn unexport(&self) ->  ProxyResult<()>;

	fn set_value(&self, value: u8) -> ProxyResult<()>;

	fn set_direction(&self, dir: DirectionProxied) ->  ProxyResult<()>;
}

#[cfg(linux)]
pub mod gpioaccess{

	extern crate sysfs_gpio;
	use sysfs_gpio::{Direction, Pin};
	use super::{PinProxyContract,DirectionProxied,ProxyError,ProxyResult};

	pub struct PinProxy {
		pin : Pin,
	}

	impl PinProxyContract for PinProxy
	{
		fn new(pin_num: u64) -> PinProxy {
			PinProxy{ pin: Pin::new(pin_num) }
		}

		fn export(&self) ->  ProxyResult<()>{
			match self.pin.export()
			{
				Ok(()) => Ok(()),
				_ => ProxyError::MonErreur,

			}
		}

		fn unexport(&self) ->  ProxyResult<()> {
			match self.pin.unexport()
			{
				Ok(()) => Ok(()),
				_ => yProxyError::MonErreur,
			}
		}

		fn set_value(&self, value: u8) ->  ProxyResult<()> {
			match self.pin.set_value(value)
			{
				Ok(()) => Ok(()),
				_ => ProxyError::MonErreur,
			}
		}

		fn set_direction(&self, dir: DirectionProxied) ->  ProxyResult<()>{
			let dirmapped =  match dir {
                                           DirectionProxied::In =>  Direction::In,
                                           DirectionProxied::Out => Direction::Out,
                                           DirectionProxied::High => Direction::High,
                                           DirectionProxied::Low => Direction::Low,
                                       };
			match self.pin.set_direction(dirmapped)
			{
				Ok(()) => Ok(()),
				_ => ProxyError::MonErreur,
			}
		}

    }
}

#[cfg(not(linux))]
pub mod gpioaccess{
	use std::fmt;
	use super::{PinProxyContract,DirectionProxied,ProxyError,ProxyResult};

	pub struct PinProxy {
		pin_num: u64,
	}

	impl PinProxyContract for PinProxy
	{
		fn new(pin_num: u64) -> PinProxy {
			PinProxy{ pin_num: pin_num }
		}
		fn export(&self) -> ProxyResult<()>
		{
			Ok::<(),ProxyError>(())
		}
		fn unexport(&self) -> ProxyResult<()>
		{
			Ok::<(),ProxyError>(())
		}

		fn set_value(&self, value: u8) -> ProxyResult<()>
		{
			Ok::<(),ProxyError>(())
		}

		fn set_direction(&self, dir: DirectionProxied) -> ProxyResult<()>
		{
			Ok::<(),ProxyError>(())
		}
    }
}


#[derive(Copy, Clone)]
pub struct Sys;
impl Key for Sys { type Value = ActorSystem; }

struct Resolver;

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
    fn new(_dummy: ()) -> Resolver {
        Resolver
    }
}

fn main() {
	env_logger::init().unwrap();
	let dragon_actor_system  = ActorSystem::new("dragon".to_owned());
	dragon_actor_system.spawn_threads(3);

	dragon_actor_system.actor_of(Props::new(Arc::new(Dragon::new),()), "gorynich".to_owned());
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
					Ok(Response::with((status::Ok, dragon_event.to_string())))
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
	Iron::new(mount).http("0.0.0.0:8080").unwrap();
}

#[derive(Copy, Clone, PartialEq,Debug)]
enum DragonCommands {
	MoveWings,
	OpenMouth,
	BlinkEyes,
}

#[derive(Copy, Clone, PartialEq,Debug)]
enum DragonEvents {
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

struct Dragon;

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
	fn new(_: ()) -> Dragon {
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
		   context.actor_of(Props::new(Arc::new(PinActor::new), 18), "pin18".to_owned()).unwrap();
    }
    fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		if let Ok(_message) = Box::<Any>::downcast::<LimbCommands>(_message){
			match *_message {
				LimbCommands::Init(max) => { println!("Initializing with maximum {}",max); },
				LimbCommands::Aggitate => {
            let pin18 : ActorRef = _context.children().get(&ActorPath::new_local("/user/gorynich/wings/pin18".to_owned())).cloned().unwrap();
						_context.tell(pin18,PinCommands::Blink(20));
						println!("Moving Wings");
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
    fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		if let Ok(_message) = Box::<Any>::downcast::<LimbCommands>(_message){
			match *_message {
				LimbCommands::Init(max) => { println!("Initializing with maximum {}",max); },
				LimbCommands::Aggitate => {
						println!("Blinking Eyes");
						//_context.complete(_context.sender(),LimbEvents::Aggitated);
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


#[derive(Copy, Clone, PartialEq,Debug)]
enum PinCommands {
	Blink(u64),
	Switch,
}

struct PinActor {
	pinproxy :  Mutex<PinProxy>,
}

impl Actor for PinActor {
    fn receive(&self, _message: Box<Any>, _context: ActorCell) {
		if let Ok(_message) = Box::<Any>::downcast::<PinCommands>(_message){
			match *_message {
				  PinCommands::Blink(times) => {
              let pin = self.pinproxy.lock().unwrap();
              match pin.export() {
                  Ok(()) => {
						          println!("Pin exported");
                      let test = try!(pin.set_direction(DirectionProxied::Out));
                      let test2 = try!({
                          pin.set_direction(DirectionProxied::High);
                          thread::sleep(1);
                          pin.set_direction(DirectionProxied::Low);
                          thread::sleep(1);
                      });
                      match pin.unexport() {
                          Ok(()) => {
						                  println!("Pin unexported");
                          }
                          _ => {}
                      }
                  }
                  _ => {}
              }
          },
				PinCommands::Switch => {	}
			}
		}
	}
}

impl PinActor{
	fn new(pin_number: u64) -> PinActor {
		PinActor{ pinproxy : Mutex::new(PinProxy::new(pin_number)) }
    }
}

mod test;
