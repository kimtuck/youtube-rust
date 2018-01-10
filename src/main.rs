extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
extern crate reqwest;

use hyper::{Client};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::{Future, Stream};
use std::io::{self, Write};
use reqwest::{Request, Response, Error};

fn main() {

    let youtube_url = String::from("https://www.youtube.com/watch?v=ycU56DnjB3I&feature=youtu.be");
    //let youtube_url = String::from("https://www.youtube.com");
    //println!("{}",youtube_url);
    let mut response = youtube_html_source(youtube_url);
    println!("{}", response);

    println!("done calling youtube_html_source");
}

fn youtube_html_source(youtube_url: String) -> String {
    let mut resp = reqwest::get(youtube_url.as_str()).unwrap();

    assert!(resp.status().is_success());

    let body = resp.text().unwrap();

    //println!("body = {:?}", body);

    body
}
