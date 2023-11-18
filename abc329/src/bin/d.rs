use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut president = (0, 0);
    let mut cnt = vec![0; n + 1];
    for &a_i in &a {
        cnt[a_i] += 1;

        if president.0 < cnt[a_i] || (president.0 == cnt[a_i] && president.1 > a_i) {
            president = (cnt[a_i], a_i);
            println!("{}", a_i);
        } else {
            println!("{}", president.1);
        }
    }
}
