use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        mut ab: [(usize,usize); n],
    }

    let mut ok = false;
    // next_permutation()
    permutohedron::heap_recursive(&mut ab, |ab| {
        let mut field = vec![vec![false; w]; h];
        if rec(&mut field, h, w, ab, 0) {
            ok = true;
        }
    });

    println!("{}", if ok { "Yes" } else { "No" });
}

fn rec(field: &mut Vec<Vec<bool>>, h: usize, w: usize, ab: &[(usize, usize)], i: usize) -> bool {
    if all_filled(field) {
        return true;
    }

    if i == ab.len() {
        return false;
    }

    let (a, b) = ab[i];
    let (x, y) = first_empty_cell(field, h, w).unwrap();
    if put_block(field, h, w, x, y, a, b) {
        if rec(field, h, w, ab, i + 1) {
            return true;
        }
        remove_block(field, x, y, a, b);
    }
    if put_block(field, h, w, x, y, b, a) {
        if rec(field, h, w, ab, i + 1) {
            return true;
        }
        remove_block(field, x, y, b, a);
    }
    false
}

fn all_filled(field: &Vec<Vec<bool>>) -> bool {
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if !field[i][j] {
                return false;
            }
        }
    }
    true
}

fn first_empty_cell(field: &Vec<Vec<bool>>, h: usize, w: usize) -> Option<(usize, usize)> {
    for i in 0..h {
        for j in 0..w {
            if !field[i][j] {
                return Some((i, j));
            }
        }
    }
    None
}

fn put_block(
    field: &mut Vec<Vec<bool>>,
    h: usize,
    w: usize,
    x: usize,
    y: usize,
    a: usize,
    b: usize,
) -> bool {
    for i in x..x + a {
        for j in y..y + b {
            if i >= h || j >= w || field[i][j] {
                return false;
            }
        }
    }
    for i in x..x + a {
        for j in y..y + b {
            field[i][j] = true;
        }
    }
    true
}

fn remove_block(field: &mut Vec<Vec<bool>>, x: usize, y: usize, a: usize, b: usize) {
    for i in x..x + a {
        for j in y..y + b {
            field[i][j] = false;
        }
    }
}
