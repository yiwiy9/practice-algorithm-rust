use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        x: String,
        y: String,
    }

    let order = HashMap::from([
        (String::from("Ocelot"), 1),
        (String::from("Serval"), 2),
        (String::from("Lynx"), 3),
    ]);

    println!("{}", if order[&x] >= order[&y] { "Yes" } else { "No" });
}
