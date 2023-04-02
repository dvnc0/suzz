use curl::easy::Easy;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod print;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn replace_suzz_in_url(url: String, word: &str) -> Result<(), clap::error::Error> {
    let parsed_url = url.replace("suzz", word);
    let message = "Processing URL: ".to_string() + &parsed_url.to_owned();
    print::info(message.to_string());
    check_given_url(&parsed_url);
    Ok(())
}

pub fn check_given_url(url: &String) {
    let mut curl_res = Easy::new();
        curl_res.url(&url).unwrap();
        curl_res.write_function(|data| {
            Ok(data.len())
        }).unwrap();
        curl_res.perform().unwrap_or_else(|e| {
            print::error(e.to_string());
        });
    
    print::message("Response Code: ".to_string() + &curl_res.response_code().unwrap().to_string().to_owned());
}