use clap::{Command, Arg, ArgAction, value_parser};

#[derive(Debug)]
pub struct Target {
    files: String,
    url: String,
    delay: i32,
    verbose:bool
}
fn main() {
    let matches = Command::new("suzz")
        .version("0.1.0")
        .author("dvnc0")
        .about("A simple fuzzer")
        .arg(
            Arg::new("url")
                .value_name("URL")
                .help("The URL to test")
                .required(true)
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .value_name("FILE")
                .help("Word list to use")
                .required(true)
        )
        .arg(
            Arg::new("delay")
                .long("delay")
                .short('d')
                .value_name("DELAY")
                .help("Optional delay in seconds between requests")
                .value_parser(value_parser!(i32))
                .default_value("0")
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Emable verbose output")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let target = Target {
        files: matches.get_one::<String>("file").unwrap().to_string(),
        url: matches.get_one::<String>("url").unwrap().to_string(),
        delay: *matches.get_one::<i32>("delay").unwrap(),
        verbose: matches.get_flag("verbose")
    };

}