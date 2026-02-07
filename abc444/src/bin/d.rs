use itertools::Itertools;
use proconio::input;

const MAX: usize = 200_010;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut num_cnt = vec![0; MAX];
    for &a_i in &a {
        num_cnt[a_i] += 1;
    }

    let mut ans = vec![];
    let mut flow = 0;
    let mut cur_cnt = n;
    for digit in 1..MAX {
        flow += cur_cnt;

        ans.push(std::char::from_digit((flow % 10) as u32, 10).unwrap());
        flow /= 10;

        cur_cnt -= num_cnt[digit];
        if cur_cnt == 0 {
            break;
        }
    }
    ans.reverse();

    if flow > 0 {
        let flow_chars = decimal_to_chars(flow, 10);
        print!("{}", flow_chars.iter().join(""))
    }

    println!("{}", ans.iter().join(""));
}

pub fn decimal_to_chars(mut n: usize, base: usize) -> Vec<char> {
    if n == 0 {
        return vec!['0'];
    }
    let mut result = Vec::new();
    while n > 0 {
        let char = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        result.push(char);
        n /= base;
    }
    result.iter().rev().copied().collect()
}
