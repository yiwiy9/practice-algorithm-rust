use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s[3] as usize - '0' as usize == 3
        && s[4] as usize - '0' as usize == 1
        && s[5] as usize - '0' as usize == 6
    {
        println!("No");
        return;
    }

    if s[3] as usize - '0' as usize == 0
        && s[4] as usize - '0' as usize == 0
        && s[5] as usize - '0' as usize == 0
    {
        println!("No");
        return;
    }

    if s[3] as usize - '0' as usize > 3 {
        println!("No");
        return;
    }
    if (s[3] as usize - '0' as usize) < 3 {
        println!("Yes");
        return;
    }
    if s[4] as usize - '0' as usize > 4 {
        println!("No");
        return;
    }

    println!("Yes");
}
