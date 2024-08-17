use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    }

    let mut last_i = 0;
    for (i, &x_i) in x.iter().enumerate().rev() {
        last_i = i;
        if x_i == '.' {
            last_i -= 1;
            break;
        }
        if x_i != '0' {
            break;
        }
    }

    println!("{}", x.iter().take(last_i + 1).collect::<String>());
}
