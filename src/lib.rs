extern crate iron;
extern crate router;
extern crate staticfile;
extern crate handlebars_iron;

use iron::prelude::*;
use handlebars_iron::HandlebarsEngine;

pub mod handlers;

pub fn main(on: &str) {
    let f = staticfile::Static::new("public");
    let mut r = router::Router::new();
    r.get("/", handlers::index, "index");
    r.get("/article/:slug", handlers::article, "article");
    r.get("/static/*", f, "static");
    let mut x = iron::Chain::new(r);
    let mut hbs = HandlebarsEngine::new();
    hbs.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    x.link_after(hbs);
    Iron::new(x).http(on).unwrap();
    println!("Listening on: {}", on);
}
