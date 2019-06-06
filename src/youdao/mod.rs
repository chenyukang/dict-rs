use hyper::Client;

use url::Url;
use hyper::rt::{self, Future, Stream};

extern crate serde_json;
use serde_json::{Value};

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
    Client::new()
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

pub fn translate(input: String)  {
    let uri = Url::parse_with_params("http://fanyi.youdao.com/openapi.do",
                                     &[
                                         ("keyfrom", "wufeifei"),
                                         ("key", "716426270"),
                                         ("type", "data"),
                                         ("doctype", "json"),
                                         ("version", "1.1"),
                                         ("q", &input)]).unwrap().into_string();
    rt::run(fetch_url(uri.parse().unwrap()).map(|v| {
        println!("result: {}", v["translation"]);
        match v["web"].as_array() {
            None => (),
            Some(others) => {
                for o in others.into_iter() {
                    println!("{k} => {v} ",
                             k = o["key"].as_str().unwrap(), v = o["value"]);
                }
            }

        }
    }).map_err(|_err| {
        eprintln!("err")
    }))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let uri = Url::parse_with_params("http://fanyi.youdao.com/openapi.do",
                                         &[
                                             ("keyfrom", "wufeifei"),
                                             ("key", "716426270"),
                                             ("type", "data"),
                                             ("doctype", "json"),
                                             ("version", "1.1"),
                                             ("q", "hello")]).unwrap().into_string();

        let parsed_uri : hyper::Uri = uri.parse().unwrap();
        assert_eq!(parsed_uri.host().unwrap(), "fanyi.youdao.com");
        assert_eq!(parsed_uri.port(), None);
        //assert_ok!(uri.parse());
    }
}
