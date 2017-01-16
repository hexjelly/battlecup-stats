use std::collections::HashMap;

use maud::{ DOCTYPE, Markup };

pub fn template(title: &str, data: &HashMap<usize, String>) -> Markup {
    html! {
        (DOCTYPE)

        html lang="en" {
            head {
              meta charset="utf-8" /
              title (title)
            }

            body {
                @for battle in data {
                    p { (battle.0) ": " (battle.1) }
                }
            }
        }
    }
}
