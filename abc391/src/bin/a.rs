use proconio::input;

fn main() {
    input! {
        d: String,
    }

    match d.as_str() {
        "N" => println!("S"),
        "E" => println!("W"),
        "W" => println!("E"),
        "S" => println!("N"),
        "NE" => println!("SW"),
        "NW" => println!("SE"),
        "SE" => println!("NW"),
        "SW" => println!("NE"),
        _ => unreachable!(),
    }
}
