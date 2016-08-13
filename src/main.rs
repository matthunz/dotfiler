extern crate tera;
extern crate toml;

use tera::{Tera, Context};
use std::fs::File;
use std::io::{Read, Write};
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
        context.add(key, &val.as_str().expect("error: value not a valid string"));
    };

    let tera = Tera::new("examples/*");

    let files = config
        .get("files")
        .expect("No [files] section found")
        .as_table().expect("[files] is not valid a TOML table");

    for (template, path) in files {
        let path = path.as_str().expect("error: path not a valid string");
        let render = tera
            .render(template, context.clone())
            .expect("error: couldn't render template");
        let mut file = File::create(&path)
            .expect(format!("Couldn't access {}", path).as_str());
        file.write_all(render.as_bytes());
    };
}
