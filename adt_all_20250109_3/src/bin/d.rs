use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [Usize1; k],
    }

    let mut ok = false;
    let max_a = *a.iter().max().unwrap();
    let mut set = std::collections::BTreeSet::from_iter(b.iter().copied());
    for (i, &a_i) in a.iter().enumerate() {
        if a_i == max_a && set.contains(&i) {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
