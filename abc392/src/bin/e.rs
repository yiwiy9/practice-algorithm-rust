use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut not_used = vec![];
    for i in 0..m {
        let (a, b) = ab[i];
        if !dsu.same(a, b) {
            dsu.merge(a, b);
        } else {
            not_used.push(i);
        }
    }

    let mut leaders = HashSet::new();
    for i in 0..n {
        leaders.insert(dsu.leader(i));
    }

    let mut ans = vec![];
    for i in not_used {
        if leaders.len() == 1 {
            break;
        }

        let (a, b) = ab[i];
        let base_leader = dsu.leader(a);
        leaders.remove(&base_leader);

        let connect_leader = *leaders.iter().next().unwrap();
        leaders.remove(&connect_leader);

        ans.push((i + 1, a + 1, connect_leader + 1));

        dsu.merge(a, connect_leader);
        leaders.insert(dsu.leader(a));
    }

    println!("{}", ans.len());
    for &(i, a, b) in &ans {
        println!("{} {} {}", i, a, b);
    }
}
