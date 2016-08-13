extern crate rustache;
extern crate toml;

use rustache::{HashBuilder, render_file};
use std::io::Read;
use std::process::exit;

fn main() {
    let toml = r#"
        [test]
        foo = "bar"
    "#;

    let config = match toml::Parser::new(toml).parse() {
        Some(config) => config,
        None => {
            println!("error: could not parse configuration file");
            exit(1);
        }
    };

    println!("{:?}", config.get("test").unwrap());
    
    let data = HashBuilder::new()
        .insert_string("fg", "#888888");
    
    let mut rendered = String::new();
    match rustache::render_file("examples/xresources", data) {
        Ok(mut render) => {
            render.read_to_string(&mut rendered);
        },
        Err(err) => {
            println!("error: could not read template file");
            exit(1);
        }
    };
    
    println!("{}", rendered);
}
