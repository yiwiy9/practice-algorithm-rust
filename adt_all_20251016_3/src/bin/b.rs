use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ex_s = String::from("AtCoder");
    let ex_t = String::from("Land");

    println!("{}", if s == ex_s && t == ex_t { "Yes" } else { "No" });
}
