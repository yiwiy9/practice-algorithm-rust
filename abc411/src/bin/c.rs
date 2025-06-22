use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; q],
    }

    let mut ans = 0;
    let mut math = vec![0; n];
    for &a_i in &a {
        if math[a_i] == 0 {
            if (a_i == 0 || math[a_i - 1] == 0) && (a_i == n - 1 || math[a_i + 1] == 0) {
                ans += 1;
            } else if a_i > 0 && math[a_i - 1] == 1 && a_i < n - 1 && math[a_i + 1] == 1 {
                ans -= 1;
            }
            math[a_i] = 1;
        } else {
            if (a_i == 0 || math[a_i - 1] == 0) && (a_i == n - 1 || math[a_i + 1] == 0) {
                ans -= 1;
            } else if a_i > 0 && math[a_i - 1] == 1 && a_i < n - 1 && math[a_i + 1] == 1 {
                ans += 1;
            }
            math[a_i] = 0;
        }
        println!("{}", ans);
    }
}
