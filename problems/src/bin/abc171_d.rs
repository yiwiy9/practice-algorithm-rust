use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
    }

    let mut cnt_vec = vec![0; 100_001];
    for &a_i in &a {
        cnt_vec[a_i as usize] += 1;
    }

    let mut ans: i64 = a.iter().sum();

    for _ in 0..q {
        input! {
            b: usize,
            c: usize,
        }
        let change_cnt = cnt_vec[b];
        cnt_vec[c] += change_cnt;
        cnt_vec[b] -= change_cnt;
        ans += (c as i64 - b as i64) * change_cnt;
        println!("{}", ans);
    }
}
