use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut red_field = vec![vec![false; w]; h];
    let mut uf: UnionFind<usize> = UnionFind::new(h * w);

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    r: Usize1,
                    c: Usize1,
                }
                red_field[r][c] = true;
                let uf_idx: usize = get_idx(r, c, w);

                if r > 0 && red_field[r - 1][c] {
                    uf.union(uf_idx, get_idx(r - 1, c, w));
                }
                if r + 1 < h && red_field[r + 1][c] {
                    uf.union(uf_idx, get_idx(r + 1, c, w));
                }
                if c > 0 && red_field[r][c - 1] {
                    uf.union(uf_idx, get_idx(r, c - 1, w));
                }
                if c + 1 < w && red_field[r][c + 1] {
                    uf.union(uf_idx, get_idx(r, c + 1, w));
                }
            }
            2 => {
                input! {
                    ra: Usize1,
                    ca: Usize1,
                    rb: Usize1,
                    cb: Usize1,
                }

                println!(
                    "{}",
                    if red_field[ra][ca]
                        && red_field[rb][cb]
                        && uf.equiv(get_idx(ra, ca, w), get_idx(rb, cb, w))
                    {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}

fn get_idx(r: usize, c: usize, w: usize) -> usize {
    r * w + c
}
