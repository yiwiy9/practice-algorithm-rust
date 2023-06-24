use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;
use std::collections::HashMap;

fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        ab: [(Usize1,Usize1); n],
    }

    let mut rows = BTreeSet::new();
    let mut columns = BTreeSet::new();

    for (a, b) in &ab {
        rows.insert(a);
        columns.insert(b);
    }

    let mut rows_compressor = HashMap::new();
    let mut columns_compressor = HashMap::new();

    for (i, &row) in rows.iter().enumerate() {
        rows_compressor.insert(*row, i + 1);
    }
    for (i, &column) in columns.iter().enumerate() {
        columns_compressor.insert(*column, i + 1);
    }

    for (a, b) in &ab {
        println!(
            "{} {}",
            rows_compressor.get(a).unwrap(),
            columns_compressor.get(b).unwrap()
        );
    }
}
