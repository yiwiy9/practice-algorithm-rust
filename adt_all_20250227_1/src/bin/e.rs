use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize,usize); q],
    }

    let mut query_map = std::collections::HashMap::new();
    let mut cnt_map = std::collections::HashMap::new();
    for (i, &a) in a.iter().enumerate() {
        if let Some(&cnt) = cnt_map.get(&a) {
            cnt_map.insert(a, cnt + 1);
            query_map.insert((a, cnt + 1 as usize), i);
        } else {
            cnt_map.insert(a, 1);
            query_map.insert((a, 1), i);
        }
    }

    for (x, k) in xk {
        if let Some(&i) = query_map.get(&(x, k)) {
            println!("{}", i + 1);
        } else {
            println!("-1");
        }
    }
}
