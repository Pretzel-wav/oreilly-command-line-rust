use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Justin macak")
        .about("Rust version of cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Paths to input files")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Number lines in the output")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Number only nonblank lines, ignoring blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: match matches.values_of_lossy("files") {
            None => Vec::new(),
            Some(vec) => vec,
        },
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}


pub fn run(config: Config) -> MyResult<()> {
    dbg!("{}", config);
    Ok(())
}
