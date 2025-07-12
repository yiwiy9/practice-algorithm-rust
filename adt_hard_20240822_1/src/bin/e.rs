use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = std::collections::HashMap::new();
    for string in s {
        if map.contains_key(&string) {
            let count = map.get(&string).unwrap();
            println!("{}({})", string, count);
            map.insert(string, count + 1);
        } else {
            println!("{}", string);
            map.insert(string, 1);
        }
    }
}
