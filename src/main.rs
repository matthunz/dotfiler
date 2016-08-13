extern crate rustache;

use rustache::{HashBuilder, render_file};
use std::io::Read;
use std::process;

fn main() {
    let data = HashBuilder::new()
        .insert_string("fg", "#888888");
    
    let mut rendered = String::new();
    match rustache::render_file("examples/xresources", data) {
        Ok(mut render) => {
            render.read_to_string(&mut rendered);
        },
        Err(err) => {
            println!("error: could not read template file");
            process::exit(1);
        }
    };
    
    println!("{}", rendered);
}
