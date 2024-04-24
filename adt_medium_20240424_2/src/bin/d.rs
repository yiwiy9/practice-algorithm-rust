use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut cur = s[0] as usize - 'A' as usize;
    for &c in &s {
        let a = c as usize - 'A' as usize;
        if a == cur {
            continue;
        }
        if a < cur {
            println!("No");
            return;
        }
        cur = a;
    }

    println!("Yes");
}
