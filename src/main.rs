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
		.mount("/",Static::new(Path::new("../../static/index.html")))
		.mount("/api/",router)
		.mount("/react/",Static::new(Path::new("../../bower_components/react")));
	Iron::new(mount).http("192.168.0.238:3000").unwrap();
	println!("On 3000");
}
