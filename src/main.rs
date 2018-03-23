extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io;
use std::thread::{self, JoinHandle};
use futures::Future;
use futures::sync::mpsc;
use futures::Stream;
use futures::Sink;
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};
use hyper::client::HttpConnector;
use hyper::Uri;

struct Service {
    channel: mpsc::Sender<String>,
    _thread: JoinHandle<()>,
}
impl Service {
    fn new() -> Service {
        let (tx,rx)= mpsc::channel(10);
        let thread = thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let handle = core.handle();
            //let client = Client::new(&core.handle());
            let client = Client::configure()
                .connector(HttpConnector::new(4,&handle))
                .build(&handle);
            let f = rx.for_each(|xml:String|{
                let uri: Uri = "http://www.asmsoft.ru/xml/1.php".parse().unwrap();
                let mut req = Request::new(Method::Post, uri.clone());
                req.headers_mut().set(ContentType::xml());
                req.headers_mut().set(ContentLength(xml.len() as u64));
                req.set_body(xml);
                let post = client.request(req).map(|res| {
                    println!("Got result1: {:?}", res);
                    ()
                }).map_err(drop);
                post
            });

            core.run(f).unwrap();
        });
        Service {
            _thread: thread,
            channel: tx
        }

    }
    fn handle_commit(&self, xml: String) {
        self.channel.clone().send(xml).wait().unwrap();
    }
}


fn main() {
    let service = Service::new();
    service.handle_commit("<xml>".to_owned());
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
