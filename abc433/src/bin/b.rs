use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for (i, &a_i) in a.iter().enumerate() {
        let mut ans = -1;
        for j in 0..i {
            if a_i < a[j] {
                ans = j as i32 + 1;
            }
        }
        println!("{}", ans);
    }
}
