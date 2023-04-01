use clap::{Command, Arg, ArgAction};

fn main() {
    let _matches = Command::new("suzz")
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
}
