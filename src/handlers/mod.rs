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

#[derive(Debug)]
pub enum LoadError {
    IOError(io::Error),
    GlobError(GlobError),
    DecodeError(DecoderError),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Article {
    pub mdata: Metadata,
    pub contents: String,
}

impl Article {
    pub fn new() -> Article {
        Article { 
            mdata: Metadata::new(),
            contents: String::new(),
        }
    }
}

fn join<I: Iterator<Item = E>, E: ToString>(mut i: I, j: &str) -> String {
    let mut acc = match i.next() {
        Some(x) => x.to_string(),
        None    => return String::new()
    };
    for x in i {
        acc.push_str(&j);
        acc.push_str(&x.to_string());
    }
    acc
}

fn load_article(p: PathBuf) -> Result<Article, LoadError> {
    let f = try!(File::open(p).map_err(|e| LoadError::IOError(e)));
    let mut lines = BufReader::new(f).lines();
    let mut mlines = vec![];
    for line in &mut lines {
        let line = try!(line.map_err(|e| LoadError::IOError(e)));
        if &line == "" {
            break;
        }
        mlines.push(line);
    }
    let metadata: Metadata = try!(json::decode(&join(mlines.iter(), "\n")).map_err(|e| LoadError::DecodeError(e)));
    let mut contents = vec![];
    for line in lines {
        contents.push(try!(line.map_err(|e| LoadError::IOError(e))));
    }
    Ok(Article { mdata: metadata, contents: join(contents.iter(), "\n")})
}

pub fn load_articles<P: Iterator<Item = GlobResult>>(fnames: P) -> Result<Vec<Article>, LoadError> {
    let mut articles = vec![];
    for fname in fnames {
        let article = try!(load_article(try!(fname.map_err(|e| LoadError::GlobError(e)))));
        articles.push(article);
    }
    Ok(articles)
}

pub fn articles_map(articles: &[Article]) -> HashMap<String, Article> {
    let mut hm = HashMap::new();
    for article in articles.iter() {
        hm.insert(article.mdata.slug.clone(), article.clone());
    }
    hm
}

impl ToJson for Article {
    fn to_json(&self) -> Json {
        let mut hm = HashMap::new();
        hm.insert("mdata".to_owned(), self.mdata.to_json());
        hm.insert("contents".to_owned(), self.contents.to_json());
        hm.to_json()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Metadata {
    pub title: String,
    pub slug: String,
    pub author: String,
    pub date: String,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata { 
            title: String::new(),
            author: String::new(),
            slug: String::new(),
            date: String::new(),
        }
    }
}

impl ToJson for Metadata {
    fn to_json(&self) -> Json {
        let mut hm = HashMap::new();
        hm.insert("title".to_owned(), self.title.to_json());
        hm.insert("author".to_owned(), self.author.to_json());
        hm.insert("slug".to_owned(), self.slug.to_json());
        hm.insert("date".to_owned(), self.date.to_json());
        hm.to_json()
    }
}

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
