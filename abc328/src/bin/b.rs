use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let mut ans = 0;
    for (i, &d_i) in d.iter().enumerate() {
        let mut ok = true;
        let mut m = i + 1;
        let num = m % 10;

        while m / 10 > 0 {
            if m % 10 != num {
                ok = false;
                break;
            }
            m /= 10;
        }
        if m % 10 != num {
            ok = false;
        }

        if !ok {
            continue;
        }

        for mut d_m in 1..=d_i {
            ok = true;

            while d_m / 10 > 0 {
                if d_m % 10 != num {
                    ok = false;
                    break;
                }
                d_m /= 10;
            }
            if d_m % 10 != num {
                ok = false;
            }

            if ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
