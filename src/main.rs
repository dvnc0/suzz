use suzz::{Target, build_app};
use core::time;
use std::thread::sleep;
mod run;
fn main() {
    let target: Target = build_app();
    if let Ok(lines) = run::read_lines(target.files) {
        for line in lines {
            if let Ok(suzz) = line {
                run::replace_suzz_in_url(target.url.to_string(), &suzz.to_string()).expect("Error with request");
                sleep(time::Duration::from_secs(target.delay));
            }
        }
    }
}