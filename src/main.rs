extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
	fn hello_world(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello World!")))
	}
	
	Iron::new(hello_world).http("192.168.0.238:3000").unwrap();
	println!("On 3000");
}
