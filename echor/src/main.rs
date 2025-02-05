use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Justin Macak <justin.macak@gmail.com>")
        .about("echo in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = match matches.values_of_lossy("text") {
        None => String::new(),
        Some(vec) => vec.join(" "),
    };

    if matches.is_present("omit_newline") {
        print!("{}", text);
    } else {
        println!("{}", text);
    }
}