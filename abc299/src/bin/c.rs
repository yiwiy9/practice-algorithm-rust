use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut all_o: bool = true;
    let mut all_l: bool = true;
    for &c in &s {
        match c {
            'o' => {
                all_l = false;
            }
            '-' => all_o = false,
            _ => unreachable!(),
        }
    }
    if all_o || all_l {
        println!("{}", -1);
        return;
    }

    let mut ans = 0;
    let mut count = 0;
    for &c in &s {
        match c {
            'o' => {
                count += 1;
                ans = ans.max(count);
            }
            '-' => {
                count = 0;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
