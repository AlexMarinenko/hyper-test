extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate core;

use std::io;
use futures::Future;
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};
use hyper::client::HttpConnector;
use hyper::Uri;

fn main() {

    let mut core = Core::new().unwrap();
    let handle = core.handle();


    //let client = Client::new(&core.handle());
    let client = Client::configure()
        .connector(HttpConnector::new(4,&handle))
        .build(&handle);

    let uri: Uri = "http://www.asmsoft.ru/xml/1.php".parse().unwrap();

    let xml = "<xml>";

    let mut req = Request::new(Method::Post, uri.clone());
    req.headers_mut().set(ContentType::xml());
    req.headers_mut().set(ContentLength(xml.len() as u64));
    req.set_body(xml);
    let post = client.request(req).map(|res| {
       println!("Got result1: {:?}", res);
       res
    });

    core.run(post).unwrap();

    println!("Finish.");

    println!("Press enter....");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }



}
