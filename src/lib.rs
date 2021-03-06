extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate rustc_serialize;
extern crate handlebars_iron;
extern crate glob;

use glob::glob;
use iron::prelude::*;
use handlebars_iron::{HandlebarsEngine, DirectorySource};

pub mod model;
pub mod handlers;

pub fn main(on: &str) {
    let articles = model::load_articles(glob("articles/*.html").unwrap()).unwrap();
    let article_map = model::articles_map(&articles);
    let mut m = mount::Mount::new();
    let mut r = router::Router::new();
    r.get("/", move |r: &mut Request| handlers::index(&articles, r), "index");
    r.get("/article/:slug", move |r: &mut Request| handlers::article(&article_map, r), "article");
    m.mount("/", r);
    let f = staticfile::Static::new("public");
    m.mount("/static/", f);
    let mut x = iron::Chain::new(m);
    let mut hbs = HandlebarsEngine::new();
    hbs.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    hbs.reload().unwrap();
    x.link_after(hbs);
    Iron::new(x).http(&on).unwrap();
    println!("Listening on: {}", on);
}
