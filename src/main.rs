extern crate iron;
extern crate router;

mod config;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main () {
	let mut router = Router::new();

	router.get("/", | _ : &mut Request | {
		Ok(Response::with((status::Ok, "asdf")))
	});

	Iron::new(router).http(config::HOST).unwrap();
}
