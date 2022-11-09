use std::env;
extern crate reqwest;
use clap::{App, Arg};
use subtitle::{error::Result, run, arguments};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("file")
                .index(1)
                .value_name("FILE(S)")
                .about("Get subtitles for a file or multiple files.")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .short('l')
                .value_name("LANGUAGE")
                .about("The preferred language to download the subtitles in.")
                .takes_value(true)
                .default_value("en"),
        )
        .arg(
            Arg::new("dont_use_lang_fallback")
                .long("dont-use-lang-fallback")
                .short('d')
                .value_name("VALUE")
                .takes_value(false)
                .about(
                    "When used, app will stop if subtitles aren't found for the selected language.",
                ),
        )
        .get_matches();

    let result = run(arguments::Arguments::new(&matches));
    println!("{}", result.unwrap());
    return Ok(());
}
