extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate robots;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::Path;

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

	// the mounter for static files
	let mut mount = Mount::new();
	mount
		.mount("/",Static::new(Path::new("static")))
		.mount("/api/",router);
	Iron::new(mount).http("0.0.0.0:3000").unwrap();
}
