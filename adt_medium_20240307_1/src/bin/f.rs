use num::clamp;
use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

/**
 * https://atcoder.jp/contests/adt_medium_20240307_1/editorial/5137
 * https://atcoder.jp/contests/adt_medium_20240307_1/tasks/abc275_c
 * 正方形の数え上げは重複しないようにsetで管理して全探索
 */
fn main() {
    input! {
        filed: [Chars; 9],
    }

    let is_valid = |x: i32, y: i32| {
        clamp(x, 0, 8) == x && clamp(y, 0, 8) == y && filed[x as usize][y as usize] == '#'
    };

    let mut set = BTreeSet::new();
    for i in 0..9 {
        for j in 0..9 {
            for dx in 0..=8 {
                for dy in 0..=8 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let i2 = i + dx;
                    let j2 = j + dy;
                    let i3 = i2 + dy;
                    let j3 = j2 - dx;
                    let i4 = i3 - dx;
                    let j4 = j3 - dy;
                    if is_valid(i, j) && is_valid(i2, j2) && is_valid(i3, j3) && is_valid(i4, j4) {
                        let mut square = BTreeSet::new();
                        square.insert((i, j));
                        square.insert((i2, j2));
                        square.insert((i3, j3));
                        square.insert((i4, j4));
                        set.insert(square);
                    }
                }
            }
        }
    }

    println!("{}", set.len());
}
