use proconio::input;

fn main() {
    input! {
        a: [usize; 4],
    }

    let mut map = std::collections::HashMap::new();
    for &a_i in a.iter() {
        *map.entry(a_i).or_insert(0) += 1;
    }

    println!(
        "{}",
        if map.len() == 2
            && (map.iter().next().unwrap().1 == &1
                || map.iter().next().unwrap().1 == &2
                || map.iter().next().unwrap().1 == &3)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
