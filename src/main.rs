#![feature(plugin)]
#![plugin(maud_macros)]

#[macro_use]
extern crate clap;
extern crate maud;

mod fetch;
mod template;

use std::collections::HashMap;

use fetch::*;
use template::template;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut battle_data = HashMap::new();
    battle_data.insert(666, "test :^)".to_string());
    let _ = get_battle_result(666);
    println!("{}", template("test?", &battle_data).into_string());
}
