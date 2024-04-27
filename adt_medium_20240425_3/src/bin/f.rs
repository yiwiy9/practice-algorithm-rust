use proconio::input;

/**
 * https://atcoder.jp/contests/adt_medium_20240425_3/tasks/abc319_c
 */
fn main() {
    // 1次元で考える
    input! {
        c: [usize; 9],
    }

    // 十分列挙できる
    let check_line = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];
    let mut order = (0..9).collect::<Vec<_>>();

    let mut total = 0;
    let mut ok = 0;
    // next_permutation()
    permutohedron::heap_recursive(&mut order, |order| {
        let mut disappointed = false;
        for &(x, y, z) in &check_line {
            if c[x] == c[y] && order[x] < order[z] && order[y] < order[z] {
                disappointed = true;
                break;
            }
            if c[x] == c[z] && order[x] < order[y] && order[z] < order[y] {
                disappointed = true;
                break;
            }
            if c[z] == c[y] && order[z] < order[x] && order[y] < order[x] {
                disappointed = true;
                break;
            }
        }

        total += 1;
        if !disappointed {
            ok += 1;
        }
    });

    println!("{:.10}", ok as f64 / total as f64);
}
