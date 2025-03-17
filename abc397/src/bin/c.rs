use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut front_map = std::collections::HashMap::new();
    let mut back_map = std::collections::HashMap::new();
    for &a_i in &a {
        *back_map.entry(a_i).or_insert(0) += 1;
    }

    let mut ans = 0;
    for &a_i in a.iter().take(n - 1) {
        let back_count = *back_map.get(&a_i).unwrap();
        if back_count == 1 {
            back_map.remove(&a_i);
        } else {
            *back_map.entry(a_i).or_insert(0) -= 1;
        }

        *front_map.entry(a_i).or_insert(0) += 1;

        ans = ans.max(back_map.len() + front_map.len());
    }

    println!("{}", ans);
}
