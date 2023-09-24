use proconio::input;

/**
 * https://atcoder.jp/contests/abc321/tasks/abc321_e
 * https://atcoder.jp/contests/abc321/editorial/7267
 * 頂点 i の子孫であって、頂点 i との距離が d である頂点の番号の集合は、区間 [i×2^d,(i+1)×2^d) と [1,N] の共通区間である
 * => 頂点Xとの距離がKである頂点をYとし、XとYの最小共通祖先をZとおくと、
 *    Z を固定したときに「Xとの最小共通祖先がZであり、Xとの距離がKであるような頂点Yの数」を求めれば良い
 */
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: i64,
            x: i64,
            k: i64,
        }

        let n_depth = depth(n);
        let x_depth = depth(x);

        let mut ans = 0;
        for z_depth in 0..=x_depth {
            if x_depth - z_depth > k {
                continue;
            }
            if z_depth + (k - (x_depth - z_depth)) > n_depth {
                continue;
            }

            let left: i64; // 含む
            let right: i64; // 含まない
            if z_depth == x_depth {
                left = x << k; // xにぶら下がる左端
                right = (x + 1) << k; // xの右隣のノードの左端
            } else {
                let z = x >> (x_depth - z_depth);

                if x_depth - z_depth == k {
                    left = z;
                    right = z + 1;
                } else {
                    let z_for_x = x >> (x_depth - (z_depth + 1));
                    let z_for_not_x = if z_for_x % 2 == 0 {
                        z_for_x + 1
                    } else {
                        z_for_x - 1
                    };

                    left = z_for_not_x << (k - (x_depth - z_depth) - 1);
                    right = (z_for_not_x + 1) << (k - (x_depth - z_depth) - 1);
                }
            }

            if left > n {
                continue;
            }
            ans += right.min(n + 1) - left;
        }
        println!("{}", ans);
    }
}

fn depth(mut n: i64) -> i64 {
    let mut d = 0;
    while n / 2 > 0 {
        n /= 2;
        d += 1;
    }
    d
}
