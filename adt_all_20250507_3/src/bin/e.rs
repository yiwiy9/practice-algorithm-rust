use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut map = std::collections::HashMap::new();
    map.insert(x, 1);

    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }

        let mut new_map = std::collections::HashMap::new();
        for (&num, &count) in &map {
            for &a_i in &a {
                if num < a_i || num % a_i != 0 {
                    continue;
                }
                let next_num = num / a_i;
                *new_map.entry(next_num).or_insert(0) += count;
            }
        }
        map = new_map;
    }

    println!("{}", map.get(&1).unwrap_or(&0));
}
