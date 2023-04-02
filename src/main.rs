use suzz::{Target, build_app};
mod run;
fn main() {
    let target: Target = build_app();
    // need to loop through word file ...
    run::replace_suzz_in_url(target.url, "admin").expect("Error with request");
}