use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..=((n as f64).sqrt() as usize) {
        let cur_num = n / i;
        ans += cur_num;
        if i < cur_num {
            ans += (cur_num - n / (i + 1)) * i;
        }
    }

    println!("{}", ans);
}
