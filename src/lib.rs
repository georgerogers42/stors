extern crate iron;
extern crate router;
extern crate staticfile;
extern crate rustc_serialize;
extern crate handlebars_iron;
extern crate glob;

use glob::glob;
use iron::prelude::*;
use handlebars_iron::{HandlebarsEngine, DirectorySource};

pub mod handlers;

pub fn main(on: &str) {
    let f = staticfile::Static::new("public");
    let mut r = router::Router::new();
    let articles = handlers::load_articles(glob("articles/*.html").unwrap()).unwrap();
    let article_map = handlers::articles_map(&articles);
    r.get("/", move |r: &mut Request| handlers::index(&articles, r), "index");
    r.get("/article/:slug", move |r: &mut Request| handlers::article(&article_map, r), "article");
    r.get("/static/*", f, "static");
    let mut x = iron::Chain::new(r);
    let mut hbs = HandlebarsEngine::new();
    hbs.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    x.link_after(hbs);
    Iron::new(x).http(on).unwrap();
    println!("Listening on: {}", on);
}
