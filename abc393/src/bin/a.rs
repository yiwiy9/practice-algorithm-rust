use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    println!(
        "{}",
        if &s1 == "fine" && &s2 == "fine" {
            4
        } else if &s1 == "fine" && &s2 == "sick" {
            3
        } else if &s1 == "sick" && &s2 == "fine" {
            2
        } else {
            1
        }
    );
}
