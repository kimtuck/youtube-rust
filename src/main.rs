// The crates we'll be used.
extern crate reqwest;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

#[macro_use]
extern crate serde_derive;

// Objects from the crates (namespace)
use regex::{Regex};
use std::collections::HashMap;
use std::io::{copy, BufReader, BufWriter};
use std::fs::File;

// our other modules.
mod video_info;


// When deserializing the json extracted from the webpage, it is loaded into this struct
#[derive(Serialize, Deserialize, Debug)]
struct YoutubeStreamsArgs
{
    url_encoded_fmt_stream_map: String,
    adaptive_fmts: String
}

#[derive(Serialize, Deserialize, Debug)]
struct YoutubeStreams
{
    args: YoutubeStreamsArgs
}

const SOURCE: &str = "https://www.youtube.com/watch?v=l49EB5_yIcc";
const DEST: &str = "music.mp3";

fn main() {

    // A variable assignment.  -- type inference; immutable
    let youtube_url = String::from(SOURCE);

    // A call to a method.  -- ownership/consuming youtube_url
    let html = youtube_html_source(youtube_url);

    let json_str = extract_json_from_html(html);
    //println!("json: {}",json);

    // parse json.  Magic hydrating a struct from parsed json.
    let youtube_streams_as_json: YoutubeStreams = serde_json::from_str(json_str.as_str()).unwrap();

    // Collect streams info from two properties of the struct: url_encoded_fmt_stream_map, adaptive_fts
    // Vec is List<> in C#
    let streams=youtube_streams_as_json.args.url_encoded_fmt_stream_map.as_str();
    let mut streams: Vec<&str> = streams.split(",").collect();

    let strx = youtube_streams_as_json.args.adaptive_fmts.as_str();
    let mut vecs2: Vec<&str> = strx.split(",").collect();
    streams.append(&mut vecs2);
    //println!("strems: {:?}", streams);

    let infos = video_info::VideoInfo::defaults();

    // each vec is a string; a url.  We convert the url to a list of tuples, each tuple is a name/value pair.
    // Then we convert the list of name/value pairs to a hashmap.  We end up with a collection (Vec) of hashmaps.
    //
    // Then we look through the hashmaps for the first one that is an audio stream; and pick out the url for that audio stream.
    // match
    // result of the entire expression is the return value from match
    let audio_stream_info = streams.iter().map(|extraction_info| {
        let vec: Vec<(String, String)> = serde_urlencoded::de::from_str(extraction_info).unwrap();
        let hashmap_parsed_url: HashMap<String, String> = vec.iter().cloned().collect();
        hashmap_parsed_url
    }).find(|hashmap| {
        //println!("hashmap: {:?}", hashmap);
        let itag: i32 = hashmap.get("itag").unwrap().parse::<i32>().unwrap();
        let videoinfo = video_info::VideoInfo::find_videoinfo_for_formatcode(&infos, itag);
        //println!("itag {}--> type {:?}",itag,videoinfo.adaptive_type);
        //println!("------------------------------");
        //println!("");
        //println!("");
        match videoinfo.adaptive_type {
            video_info::AdaptiveType::Audio => true,
            _  => false
        }
    }).unwrap();
    let stream_url = audio_stream_info.get("url").unwrap();
    println!("Downloading from {}",stream_url);

    // stream bytes from stream_url into file
    let capacity=1000000;
    let response = reqwest::get(stream_url).expect("Failed to send request");
    let mut responsebuf = BufReader::with_capacity(capacity,response);

    let file = File::create(DEST).unwrap();
    let mut writebuf = BufWriter::with_capacity(capacity,file);
    copy(&mut responsebuf, &mut writebuf).expect("Failed to copy");
}

// "body" is an expression (no semicolon) -- it is the return value for the function
fn youtube_html_source(youtube_url: String) -> String {

    // mutable reference; resp is a http response object
    // unwrap; Option Some<x>, None
    let mut resp = reqwest::get(youtube_url.as_str()).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();
    //println!("body = {:?}", body);

    body
}

fn extract_json_from_html(html: String) -> String {

    //println!("html: {}",html);
    let re = Regex::new(r#"ytplayer\.config\s*=\s*(\{.+?\});"#).unwrap();
    let caps = re.captures(html.as_str()).unwrap();
    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let json = String::from(text1);
    //println!("json: {}",json);
    json
}

