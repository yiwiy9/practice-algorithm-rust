use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        ta: [(usize, usize); n],
    }

    let mut zero_accumulate = vec![0; n];
    let mut one_accumulate = vec![0; n];
    let mut zero_cur = 0;
    let mut one_cur = (1 << 30) - 1;
    for (i, &(t, a)) in ta.iter().enumerate() {
        match t {
            1 => {
                zero_cur &= a;
                one_cur &= a;
            }
            2 => {
                zero_cur |= a;
                one_cur |= a;
            }
            3 => {
                zero_cur ^= a;
                one_cur ^= a;
            }
            _ => unreachable!(),
        }
        zero_accumulate[i] = zero_cur;
        one_accumulate[i] = one_cur;
    }

    let mut x = c;
    for i in 0..n {
        let mut next_x = 0;

        for d in 0..30 {
            let is_zero = (x & (1 << d)) == 0;
            if is_zero {
                if zero_accumulate[i] & (1 << d) != 0 {
                    next_x += 1 << d;
                }
            } else if one_accumulate[i] & (1 << d) != 0 {
                next_x += 1 << d;
            }
        }

        x = next_x;
        println!("{}", x);
    }
}
