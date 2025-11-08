use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s_a: Chars,
        s_b: Chars,
        s_c: Chars,
    }

    let mut char_set = HashSet::new();
    s_a.iter().for_each(|&c| {
        char_set.insert(c);
    });
    s_b.iter().for_each(|&c| {
        char_set.insert(c);
    });
    s_c.iter().for_each(|&c| {
        char_set.insert(c);
    });

    if char_set.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let char_vec = char_set.into_iter().collect_vec();

    for p in (0..10).permutations(10) {
        let mut map = HashMap::new();
        for (i, &char) in char_vec.iter().enumerate() {
            map.insert(char, p[i]);
        }

        let Some(num_a) = get_num(&map, &s_a) else {
            continue;
        };
        let Some(num_b) = get_num(&map, &s_b) else {
            continue;
        };
        let Some(num_c) = get_num(&map, &s_c) else {
            continue;
        };

        if num_a + num_b == num_c {
            println!("{}", num_a);
            println!("{}", num_b);
            println!("{}", num_c);
            return;
        }
    }

    println!("UNSOLVABLE");
}

fn get_num(map: &HashMap<char, usize>, s: &[char]) -> Option<usize> {
    if map[&s[0]] == 0 {
        return None;
    }
    let mut num = map[&s[0]];
    for i in 1..s.len() {
        num *= 10;
        num += map[&s[i]];
    }
    Some(num)
}
