use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut count = vec![0; 26];
    let mut cur = 0;
    let mut before = '1';
    for &c in &s {
        if c != before {
            cur = 1;
            count[c as usize - 'a' as usize] = count[c as usize - 'a' as usize].max(cur);
            before = c;
            continue;
        }

        cur += 1;
        count[c as usize - 'a' as usize] = count[c as usize - 'a' as usize].max(cur);
        before = c;
    }

    println!("{}", count.iter().sum::<usize>());
}
