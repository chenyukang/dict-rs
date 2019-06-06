extern crate hyper;
extern crate serde;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

extern crate serde_json;
use serde_json::{Result, Value};



// Define a type so we can return multiple types of errors
enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}


fn fetch_url(url: hyper::Uri) -> impl Future<Item=Value, Error=FetchError> {
    let client = Client::new();


    client
    // Fetch the url...
        .get(url)
    // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
    // use the body after concatenation
        .and_then(|body| {
            // try to parse as json with serde_json
            let v: Value = serde_json::from_slice(&body)?;

            Ok(v)
        })
        .from_err()
}


fn main() {
    let uri = "http://fanyi.youdao.com/openapi.do?keyfrom=wufeifei&key=716426270&type=data&doctype=json&version=1.1&q=hello".parse().unwrap();
    rt::run(fetch_url(uri).map(|r| {
        println!("res: {}", r)
    }).map_err(|err| {
        eprintln!("err")
    }))
}
