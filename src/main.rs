extern crate tera;
extern crate toml;

use tera::{Tera, Context};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::path::Path;

fn parse_config(config_str: &String, theme: &String) -> (Context, toml::Table) {
    let config = match toml::Parser::new(&config_str).parse() {
        Some(config) => config,
        None => {
            println!("error: could not parse configuration file");
            exit(1);
        }
    };

    let mut context = Context::new();

    if config.contains_key("global") {
        let globals = parse_section(&config, "globals");
        for (key, val) in globals {
            context.add(key.as_str(), &val.as_str().expect("error: value not a valid string"));
        }
    }

    let variables = parse_section(&config, theme);
    for (key, val) in variables {
        context.add(key.as_str(), &val.as_str().expect("error: value not a valid string"));
    }

    let files = parse_section(&config, "files");

    (context, files)
}

fn parse_section(config: &toml::Table, section: &str) -> toml::Table {
    let result = match config.get(section).and_then(toml::Value::as_table) {
        Some(table) => table,
        None => {
            println!("[{}] section is missing or invalid", section);
            exit(1);
        }
    };
    result.clone()
}

fn render(template: &String, path: &toml::Value, tera: &Tera, context: Context) {
    let path = path.as_str().expect("error: path not a valid string");
    let render = match tera.render(template, context) {
        Ok(r) => r,
        Err(_) => {
            println!("error: could not render template: {}", template);
            return
        }
    };

    if let Err(_) = File::create(&path).and_then(|mut f| f.write_all(render.as_bytes())) {
        println!("error: could not write to {}", path);
        return
    }

    println!("Rendered {}", path);
}

fn main() {
    let app_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config) => Path::new(xdg_config.as_str()).join("dotfiler"),
        Err(_) => env::home_dir().unwrap().join(".config/dotfiler")
    };

    let mut config_file = match File::open(app_dir.join("config.toml")) {
        Ok(f) => f,
        Err(_) => {
            println!("error: could not open configuration file");
            exit(1);
        }
    };

    let mut config_str = String::new();
    if let Err(_) = config_file.read_to_string(&mut config_str) {
        println!("error: could not read configuration file");
        exit(1);
    }

    let theme = env::args().nth(1).unwrap_or("default".to_string());
    let (context, files) = parse_config(&config_str, &theme);

    let tera = Tera::new(app_dir.join("templates/*").to_str().unwrap());

    for (template, path) in files {
        render(&template, &path, &tera, context.clone());
    };
}
