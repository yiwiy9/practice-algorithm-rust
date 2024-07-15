use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }

    if c == String::from("Red") {
        println!("{}", g.min(b));
    } else if c == String::from("Green") {
        println!("{}", r.min(b));
    } else {
        println!("{}", r.min(g));
    }
}
