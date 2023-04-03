use curl::easy::Easy;
use suzz::Target;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod print;

// Read lines from word list
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Replace suzz in the url
fn replace_suzz_in_url(url: String, word: &str) -> Result<String, clap::error::Error> {
    let parsed_url = url.replace("suzz", word);
    let message = "Processing URL: ".to_string() + &parsed_url.to_owned();
    print::info(message.to_string());
    Ok(parsed_url)
}

// GET | HEAD request
fn make_request(url: &String, is_head: bool) {
    let request_type = if is_head {
        "HEAD".to_string()
    } else {
        "GET".to_string()
    };

    let mut curl_res = Easy::new();
        curl_res.url(&url).unwrap();
        curl_res.follow_location(true).unwrap();
        if is_head {
            curl_res.nobody(true).unwrap();
            curl_res.custom_request("HEAD").unwrap();
        }
        curl_res.write_function(|data| {
            Ok(data.len())
        }).unwrap();
        curl_res.perform().unwrap_or_else(|e| {
            print::error(e.to_string());
        });
    
    print::message(request_type + &" Request Response Code: ".to_string() + &curl_res.response_code().unwrap().to_string().to_owned());
}

// Decide on request type
pub fn decide_request_type(target: &Target, word: &str) {
    let target_url = &target.url;
    let url = replace_suzz_in_url(target_url.to_string(), word).unwrap();
    if target.head {
        make_request(&url, true);
    } else {
        make_request(&url, false);
    }
}