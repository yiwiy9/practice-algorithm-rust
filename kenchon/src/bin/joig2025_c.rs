use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::BTreeMap;

fn main() {
    input! {
        _n: usize,
        q: usize,
        t: Chars,
        p: [Usize1; q],
    }

    let get_next_char = |chars: &[char]| -> char {
        if chars.iter().filter(|&&c| c == 'A').count() > chars.len() / 2 {
            'A'
        } else {
            'B'
        }
    };

    let mut map = BTreeMap::new();
    let mut i = 0;
    let mut s = t.clone();
    loop {
        if s.len() == 1 {
            map.insert((i, 0), s[0]);
            break;
        }

        let mut j = 0;
        let mut next_s = vec![];
        loop {
            if s.len() <= j {
                break;
            }

            map.insert((i, j), s[j]);
            map.insert((i, j + 1), s[j + 1]);
            map.insert((i, j + 2), s[j + 2]);

            let next_char = get_next_char(&s[j..=j + 2]);
            next_s.push(next_char);
            j += 3;
        }
        s = next_s;
        i += 1;
    }

    for &p_k in &p {
        map.entry((0, p_k))
            .and_modify(|cur| if *cur == 'A' { *cur = 'B' } else { *cur = 'A' });

        let mut ii = 0;
        let mut jj = p_k;
        while ii < i {
            let jj_base = jj / 3 * 3;
            let next_char = get_next_char(&[
                *map.get(&(ii, jj_base)).unwrap(),
                *map.get(&(ii, jj_base + 1)).unwrap(),
                *map.get(&(ii, jj_base + 2)).unwrap(),
            ]);

            ii += 1;
            jj /= 3;

            map.insert((ii, jj), next_char);
        }

        println!("{}", map.get(&(i, 0)).unwrap());
    }
}
