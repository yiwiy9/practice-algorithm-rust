use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        a: Chars,
    }

    let mut a_cnt = a.iter().map(|&c| (c, 1)).collect::<Vec<_>>();
    while a_cnt.len() > 1 {
        let mut next = Vec::new();

        let end = (a_cnt.len() + 2) / 3;
        for i in 0..end {
            let mut zero_cnt = 0;
            let mut zero_min = std::usize::MAX;
            let mut one_min = std::usize::MAX;
            let mut all_cnt = 0;
            let mut all_max = 0;
            for j in 0..3 {
                let idx = i * 3 + j;
                let (c, cnt) = a_cnt[idx];
                if c == '0' {
                    zero_cnt += 1;
                    zero_min = zero_min.min(cnt);
                } else {
                    one_min = one_min.min(cnt);
                }
                all_cnt += cnt;
                all_max = all_max.max(cnt);
            }
            match zero_cnt {
                0 => {
                    next.push(('1', all_cnt - all_max));
                }
                1 => {
                    next.push(('1', one_min));
                }
                2 => {
                    next.push(('0', zero_min));
                }
                3 => {
                    next.push(('0', all_cnt - all_max));
                }
                _ => unreachable!(),
            }
        }
        a_cnt = next;
    }

    println!("{}", a_cnt[0].1);
}
