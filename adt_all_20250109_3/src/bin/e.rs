use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize,usize); n],
    }

    let mut set = std::collections::BTreeSet::new();
    set.insert(0);

    for &(a, b) in &ab {
        let mut new_set = std::collections::BTreeSet::new();
        for &s in &set {
            if s + a <= x {
                new_set.insert(s + a);
            }
            if s + b <= x {
                new_set.insert(s + b);
            }
        }
        set = new_set;
    }

    println!("{}", if set.contains(&x) { "Yes" } else { "No" });
}
