use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        a: Chars,
    }

    let mut c = a.iter().map(|&x| (x, 1)).collect::<Vec<_>>();

    while c.len() > 1 {
        let mut next_c = vec![];
        for i in (0..c.len()).step_by(3) {
            let mut zero_cnt = 0;
            let mut zero_min = std::usize::MAX;
            let mut one_min = std::usize::MAX;
            let mut all_cnt = 0;
            let mut all_max = 0;
            for j in 0..3 {
                let (c, cnt) = c[i + j];
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
                    next_c.push(('1', all_cnt - all_max));
                }
                1 => {
                    next_c.push(('1', one_min));
                }
                2 => {
                    next_c.push(('0', zero_min));
                }
                3 => {
                    next_c.push(('0', all_cnt - all_max));
                }
                _ => unreachable!(),
            }
        }
        c = next_c;
    }

    println!("{}", c[0].1);
}
