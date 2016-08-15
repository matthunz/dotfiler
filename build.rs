use std::env::home_dir;
use std::fs;

fn main() {
    let app_dir = format!("{}/.config/riceinator/", home_dir().unwrap().display());
    fs::create_dir(format!("{}templates/", app_dir));
    fs::copy("examples/templates/xresources", format!("{}templates/xresources", app_dir));
    fs::copy("examples/config.toml", format!("{}config.toml", app_dir));
    fs::copy("target/release/riceinator", "~/bin/riceinator");
}
