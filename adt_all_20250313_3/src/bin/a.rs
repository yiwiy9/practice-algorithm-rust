use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ok = true;
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 1 && c != '0' {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
