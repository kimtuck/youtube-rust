extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

use hyper::{Client};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::{Future, Stream};
use std::io::{self, Write};

fn main() {

    let youtube_url = String::from("https://www.youtube.com/watch?v=ycU56DnjB3I&feature=youtu.be");
    //println!("{}",youtube_url);
    youtube_html_source(youtube_url);
}

fn youtube_html_source(youtube_url: String) {

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let req = client.get("https://hyper.rs".parse().unwrap());
    let res = core.run(req);
    assert_eq!(res.is_ok(),false);

    /*
    //println!("{}", youtube_url);
    //let addr = youtube_url.parse().unwrap();
    //println!("{}",addr);
    //let work = client.get(addr).and_then(|res| {

    let uri = "https://www.youtube.com".parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });
    core.run(work);
    */
}

