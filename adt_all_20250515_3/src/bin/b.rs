use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = vec![];
    for &c in s.iter().rev() {
        if c == '.' {
            break;
        }
        ans.push(c);
    }
    ans.reverse();

    println!("{}", ans.iter().collect::<String>());
}
