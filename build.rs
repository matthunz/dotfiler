use std::env::home_dir;
use std::fs;
use std::path::Path;

fn main() {
    let formatted = format!("{}/.config/riceinator/", home_dir().unwrap().display());
    let app_dir = Path::new(formatted.as_str());
    if app_dir.exists() {
        println!("Dotfiler directory already exists, skipping example file copy")
    } else {
        fs::create_dir(app_dir);
        fs::create_dir(app_dir.join("templates/"));
        fs::copy("examples/templates/xresources", app_dir.join("templates/xresources"));
        fs::copy("examples/config.toml", app_dir.join("{}config.toml"));
        fs::copy("target/release/riceinator", "~/bin/riceinator");
    }
}
