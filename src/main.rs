use suzz::{Target, build_app};
use core::time;
use std::thread::sleep;
mod run;
fn main() {
    let target: Target = build_app();
    if let Ok(lines) = run::read_lines(&target.files) {
        for line in lines {
            if let Ok(suzz) = line {
                run::decide_request_type(&target, &suzz.to_string());
                if target.delay > 0 {
                    sleep(time::Duration::from_secs(target.delay));
                }
                
            }
        }
    }
}