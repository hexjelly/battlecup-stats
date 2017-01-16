#![feature(plugin)]
#![plugin(maud_macros)]

#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate maud;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod fetch;
mod template;

use std::collections::HashMap;

use fetch::get_battle_result;
use template::template;

use clap::App;

fn main() {
    // arg parsing
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let title = matches.value_of("title").unwrap_or("Battle Cup Stats");
    // split comma separated list, then convert &str to get battle id list as ints
    let battle_id_list: Vec<_> = matches.value_of("INPUT").unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();

    let mut battle_data = HashMap::new();
    battle_data.insert(666, "test :^)".to_string());
    let _ = get_battle_result(666);
    println!("{:?} - {}", battle_id_list, template(&title, &battle_data).into_string());
}
