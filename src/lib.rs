extern crate iron;
extern crate router;
extern crate staticfile;
extern crate handlebars_iron;

use iron::prelude::*;

pub mod handlers;

pub fn main(on: &str) {
    let f = staticfile::Static::new("public");
    let mut r = router::Router::new();
    r.get("/", handlers::index, "index");
    r.get("/article/:slug", handlers::article, "article");
    r.get("/static/*", f, "static");
    Iron::new(r).http(on).unwrap();
    println!("Listening on: {}", on);
}
