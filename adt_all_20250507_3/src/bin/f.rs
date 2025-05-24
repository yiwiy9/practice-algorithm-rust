use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut front_a_count = 0;
    for &c in s.iter() {
        if c == 'a' {
            front_a_count += 1;
        } else {
            break;
        }
    }

    let mut back_a_count = 0;
    for &c in s.iter().rev() {
        if c == 'a' {
            back_a_count += 1;
        } else {
            break;
        }
    }

    if front_a_count + back_a_count >= s.len() {
        println!("Yes");
        return;
    } else if front_a_count > back_a_count {
        println!("No");
        return;
    }

    let t = &s[front_a_count..s.len() - back_a_count];

    for i in 0..t.len() / 2 {
        if t[i] != t[t.len() - 1 - i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
