use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut b = vec![];
    let mut set = std::collections::BTreeSet::new();
    let mut j = n;
    for (i, &p_i) in p.iter().enumerate().rev() {
        set.insert(p_i);
        if p_i > p[i - 1] {
            continue;
        }
        set.insert(p[i - 1]);
        j = i - 1;

        let lower = *set.range(0..p[i - 1]).next_back().unwrap();
        set.remove(&lower);
        b.push(lower);

        for &p_j in set.iter().rev() {
            b.push(p_j);
        }

        break;
    }

    let ans = p[0..j].iter().chain(b.iter()).copied().collect::<Vec<_>>();

    println!("{}", ans.iter().join(" "));
}
