use proconio::input;
const N: usize = 1048576;

/**
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 * https://atcoder.jp/contests/abc228/editorial/2944
 *
 * 解法2. Union-Findの根を求めるのと似てる => 経路圧縮を行う
 */
fn main() {
    input! {
        q: usize,
        tx: [(usize,usize); q],
    }

    let mut parent: Vec<usize> = (0..N).collect(); // 自分が呼ばれた時に実際使われる場所の配列(連続の右端)
    let mut value = vec![-1; N];

    for &(t, x) in &tx {
        match t {
            1 => {
                let can_select = find(&mut parent, x % N);
                value[can_select] = x as i64;
                parent[can_select] = find(&mut parent, (can_select + 1) % N);
            }
            2 => {
                println!("{}", value[x % N]);
            }
            _ => unreachable!(),
        }
    }
}

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] == x {
        x
    } else {
        let p = parent[x];
        parent[x] = find(parent, p); // Path compression
        parent[x]
    }
}
