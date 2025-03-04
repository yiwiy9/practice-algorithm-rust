use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(usize,usize); n],
    }

    let mut map = std::collections::HashMap::new();
    for (a, c) in ac {
        let entry = map.entry(c).or_insert(a);
        if *entry > a {
            *entry = a;
        }
    }

    println!("{}", map.values().max().unwrap());
}
