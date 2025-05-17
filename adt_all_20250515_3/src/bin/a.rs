use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let h = if k < 60 { 21 } else { 22 };
    let m = if k < 60 { k } else { k - 60 };

    println!("{}:{m: >02}", h);
}
