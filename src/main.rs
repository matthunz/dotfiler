extern crate tera;
extern crate toml;

use tera::{Tera, Context};
use std::fs::File;
use std::io::Read;
use std::process::exit;

fn main() {
    let mut buffer = String::new();
    File::open("config.toml")
        .expect("Couldn't find configuration file")
        .read_to_string(&mut buffer);

    let config = match toml::Parser::new(&buffer).parse() {
        Some(config) => config,
        None => {
            println!("error: could not parse configuration file");
            exit(1);
        }
    };

    let variables = config
        .get("variables")
        .expect("No [variables] section found")
        .as_table().expect("[variables] is not valid a TOML table");
    let mut context = Context::new();

    for (key, val) in variables {
        context.add(key, &val.as_str().unwrap());
    };

    let tera = Tera::new("examples/*");
    println!("{}", tera.render("xresources", context).unwrap());
}
