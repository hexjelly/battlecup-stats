#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;

mod fetch;
mod template;

use fetch::*;
use template::template;

fn main() {
    let _ = get_battle_result();
    println!("{}", template().into_string());
}
