use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n],
    }

    f.sort_by(|a, b| b.cmp(a));

    let mut cur_price = 0;
    let mut cur_cnt = 0;
    let mut ans = 0;
    for f_i in f {
        cur_price += f_i;
        cur_cnt += 1;
        if cur_cnt == d {
            if p < cur_price {
                ans += p;
            } else {
                ans += cur_price;
            }
            cur_cnt = 0;
            cur_price = 0;
        }
    }

    if p < cur_price {
        ans += p;
    } else {
        ans += cur_price;
    }

    println!("{}", ans);
}
