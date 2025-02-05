use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Justin Macak <justin.macak@gmail.com>")
        .about("echo in Rust")
        .get_matches();
}