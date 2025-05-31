use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut map = std::collections::HashMap::new();
    let mut ans = vec![];
    for (i, &a_i) in a.iter().enumerate() {
        if let Some(&before) = map.get(&a_i) {
            ans.push(before);
        } else {
            ans.push(-1);
        }
        map.insert(a_i, i as i64 + 1);
    }

    println!("{}", ans.iter().join(" "));
}
