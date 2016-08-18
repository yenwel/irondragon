//#![feature(lookup_host)]

extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::env;
//use std::net;
//use std::io::Error;

//fn lookinguphost() -> Result<(), Error> {
//    for host in try!(net::lookup_host("rust-lang.org")) {
//        println!("found address : {}", try!(host));
//    }
//    Ok(())
//}

fn main() {
	// the router (for RESTfull actions)
	let mut router = Router::new();
	
	router.get("/hello",hello_world);
	router.post("/acceptsriddle",accept_riddle);
	//router.get("/randomriddle",random_riddle);
	//router.post("/voteriddle",vote_riddle);
	//router.post("/interactdragon",interact_dragon);
	
	fn hello_world(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello World!")))
	}

	fn accept_riddle(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Riddle accepted!")))
	}

	// the mounter for static files
	let mut mount = Mount::new();
	mount
		.mount("/",Static::new(Path::new("static")))
		.mount("/api/",router)
		.mount("/react/",Static::new(Path::new("bower_components/react")));
	let indexexists = Path::new("static/index.html").exists().to_string();
	println!("{}",indexexists);
	let p = env::current_dir().unwrap();
	println!("{}",p.display());
//	lookinguphost();
	Iron::new(mount).http("0.0.0.0:3000").unwrap();
	println!("On 3000");
}
