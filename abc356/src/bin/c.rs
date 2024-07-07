use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut c = vec![];
    let mut a = vec![];
    let mut r = vec![];
    for _ in 0..m {
        input! {
            c_i: usize,
        }
        c.push(c_i);

        let mut x = vec![0; n];
        for _ in 0..c_i {
            input! {
                y: Usize1,
            }
            x[y] = 1;
        }
        a.push(x.clone());

        input! {
            r_i: char,
        }
        r.push(r_i);
    }

    let mut ans = 0;
    for bit in 0..(1 << n) {
        let mut cur = vec![0; n];
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                cur[i] = 1;
            }
        }

        let mut ok = true;
        for (row_i, a_row) in a.iter().enumerate() {
            let mut true_cnt = 0;
            for i in 0..n {
                if cur[i] == 1 && a_row[i] == 1 {
                    true_cnt += 1;
                }
            }

            if r[row_i] == 'o' && true_cnt < k {
                ok = false;
                break;
            }
            if r[row_i] == 'x' && true_cnt >= k {
                ok = false;
                break;
            }
        }

        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
