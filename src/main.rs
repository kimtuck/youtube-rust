extern crate reqwest;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

#[macro_use]
extern crate serde_derive;

mod VideoInfo;


use regex::{Regex};
use std::collections::HashMap;
use std::io::{copy, stdout};

#[derive(Serialize, Deserialize, Debug)]
struct Youtube_streams_args
{
    url_encoded_fmt_stream_map: String,
    adaptive_fmts: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Youtube_streams
{
    args: Youtube_streams_args
}


fn main() {

    let youtube_url = String::from("https://www.youtube.com/watch?v=AjVY_GYtqNE");
    //let youtube_url = String::from("https://www.youtube.com");
    //println!("{}",youtube_url);
    let html = youtube_html_source(youtube_url);
    //println!("{}", html);

    let json = extract_json_from_html(html);
    //println!("json: {}",json);

    let youtube_streams_as_json: Youtube_streams = serde_json::from_str(json.as_str()).unwrap();
    let streams=youtube_streams_as_json.args.url_encoded_fmt_stream_map.as_str();
    let mut streams: Vec<&str> = streams.split(",").collect();

    let strx = youtube_streams_as_json.args.adaptive_fmts.as_str();
    let mut vecs2: Vec<&str> = strx.split(",").collect();
    streams.append(&mut vecs2);

    //println!("strems: {:?}", streams);

    let videoInfos = VideoInfo::VideoInfo::defaults();

    let audio_stream_info = streams.iter().map(|extraction_info| {
        let vec: Vec<(String, String)> = serde_urlencoded::de::from_str(extraction_info).unwrap();
        let hashmap_parsed_url: HashMap<String, String> = vec.iter().cloned().collect();
        hashmap_parsed_url
    }).find(|hashmap| {
        println!("hashmap: {:?}", hashmap);
        let itag: i32 = hashmap.get("itag").unwrap().parse::<i32>().unwrap();
        let videoinfo = VideoInfo::VideoInfo::find_videoinfo_for_formatcode(&videoInfos, itag);
        println!("itag {}--> type {:?}",itag,videoinfo.adaptive_type);
        println!("------------------------------");
        println!("");
        println!("");
        match videoinfo.adaptive_type {
            VideoInfo::AdaptiveType::Audio => true,
            _  => false
        }
    }).unwrap();
    let stream_url = audio_stream_info.get("url").unwrap();

    // stream bytes from stream_url into file
    let mut response = reqwest::get(stream_url).expect("Failed to send request");
    copy(&mut response, &mut stdout()).expect("Failed to read response");

}

fn video_info_from_source(extraction_info: &str) -> String
{
    let parsed_url: Vec<&str> = serde_urlencoded::de::from_str(extraction_info).unwrap();
    //println!("parsed url {:?}", parsed_url);
    let x = String::from("temp");
    x

    //string itag = HttpHelper.ParseQueryString(extractionInfo.Uri.Query)["itag"];
    //int formatCode = int.Parse(itag);
    //VideoInfo info = VideoInfo.Defaults.SingleOrDefault(videoInfo => videoInfo.FormatCode == formatCode);
    /*if (info != null)
        {
            info = new VideoInfo(info)
            {
                DownloadUrl = extractionInfo.Uri.ToString(),
                Title = videoTitle,
                RequiresDecryption = extractionInfo.RequiresDecryption
            };
        }

        else
        {
            info = new VideoInfo(formatCode)
            {
                DownloadUrl = extractionInfo.Uri.ToString()
            };
        }
        */
}


fn youtube_html_source(youtube_url: String) -> String {
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

/*fn extract_urls_from_json<'lifetime_a>(url_encoded_fmt_stream_map: &str) -> Vec<&'lifetime_a str> {

    let vecs = url_encoded_fmt_stream_map.split(",");
    let vec = vecs.collect::<Vec<&'lifetime_a str>>();
    //let vec = split.collect::<Vec<&str>>();
    //println!("streams");
    //println!("streams {:?}", vec);
    vec
}

*/

