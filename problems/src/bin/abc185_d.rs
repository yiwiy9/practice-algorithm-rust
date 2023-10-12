use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }

    a.sort();
    a.push(n + 1);

    let mut white_cnt_vec = vec![];
    let mut before_blue = 0;
    for &a_i in &a {
        if a_i - before_blue <= 1 {
            before_blue = a_i;
            continue;
        }

        white_cnt_vec.push(a_i - before_blue - 1);
        before_blue = a_i;
    }

    let k = white_cnt_vec.iter().min().unwrap_or(&0);

    let mut ans = 0;
    for &cnt in &white_cnt_vec {
        ans += cnt / k;
        if cnt % k != 0 {
            ans += 1
        }
    }

    println!("{}", ans);
}
