use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;

use glob::{GlobResult, GlobError};
use iron::prelude::*;
use iron::status;
use handlebars_iron::{Template};
use rustc_serialize::json::{DecoderError, Json, ToJson};
use rustc_serialize::json;
use router::Router;
use ::model::*;

pub fn index(articles: &[Article], _req: &mut Request) -> IronResult<Response> {
    let mut data = HashMap::new();
    data.insert("articles".to_owned(), articles.to_json());
    let mut resp = Response::new();
    resp.set_mut(status::Ok);
    resp.set_mut(Template::new("index", data.to_json()));
    Ok(resp)
}

pub fn article(articles: &HashMap<String, Article>, req: &mut Request) -> IronResult<Response> {
    let p = req.extensions.get::<Router>().unwrap();
    let slug = match p.find("slug") {
        Some(s) => {
            s
        }, None => {
            let mut resp = Response::new();
            resp.set_mut(status::NotFound);
            return Ok(resp)
        }
    };
    let mut resp = Response::new();
    let article = articles.get(slug).unwrap();
    resp.set_mut(status::Ok);
    resp.set_mut(Template::new("article", article.to_json()));
    Ok(resp)
}
