use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 2*n],
    }

    let mut ans = 0;
    let mut map = std::collections::HashMap::new();
    for i in 0..2 * n {
        if let Some(&v) = map.get(&a[i]) {
            if i - v == 2 {
                ans += 1;
            }
        } else {
            map.insert(a[i], i);
        }
    }

    println!("{}", ans);
}
