use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        field: [Chars; h],
    }

    let mut row_maps = vec![BTreeMap::new(); h];
    let mut column_maps = vec![BTreeMap::new(); w];

    for (i, row_i) in field.iter().enumerate() {
        for (j, &c_ij) in row_i.iter().enumerate() {
            row_maps[i]
                .entry(c_ij)
                .and_modify(|cur| *cur += 1)
                .or_insert(1);
            column_maps[j]
                .entry(c_ij)
                .and_modify(|cur| *cur += 1)
                .or_insert(1);
        }
    }

    loop {
        let mut row_updated_map = BTreeMap::new();
        let mut column_updated_map = BTreeMap::new();

        for row_map in &mut row_maps {
            if row_map.len() == 1 && *row_map.first_key_value().unwrap().1 > 1 {
                let (c, _) = row_map.pop_first().unwrap();
                row_updated_map
                    .entry(c)
                    .and_modify(|cur| *cur += 1)
                    .or_insert(1);
            }
        }

        for column_map in &mut column_maps {
            if column_map.len() == 1 && *column_map.first_key_value().unwrap().1 > 1 {
                let (c, _) = column_map.pop_first().unwrap();
                column_updated_map
                    .entry(c)
                    .and_modify(|cur| *cur += 1)
                    .or_insert(1);
            }
        }

        if row_updated_map.is_empty() && column_updated_map.is_empty() {
            break;
        }

        for (remove_c, remove_cnt) in &row_updated_map {
            for column_map in &mut column_maps {
                if column_map.contains_key(remove_c) {
                    let cnt = column_map.get_mut(remove_c).unwrap();
                    *cnt -= remove_cnt;
                    if *cnt == 0 {
                        column_map.remove(remove_c);
                    }
                }
            }
        }

        for (remove_c, remove_cnt) in &column_updated_map {
            for row_map in &mut row_maps {
                if row_map.contains_key(remove_c) {
                    let cnt = row_map.get_mut(remove_c).unwrap();
                    *cnt -= remove_cnt;
                    if *cnt == 0 {
                        row_map.remove(remove_c);
                    }
                }
            }
        }
    }

    let ans = row_maps
        .iter()
        .map(|m| m.values().sum::<usize>())
        .sum::<usize>();
    println!("{}", ans);
}
