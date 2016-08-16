extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
	let mut router = Router::new();
	router.get("/",hello_world);
	router.post("/dragonacceptsriddle",accept_riddle);
	fn hello_world(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello World!")))
	}
	fn accept_riddle(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Riddle accepted!")))
	}
	Iron::new(router).http("192.168.0.238:3000").unwrap();
	println!("On 3000");
}
