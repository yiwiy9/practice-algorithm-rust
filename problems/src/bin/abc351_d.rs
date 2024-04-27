use ac_library::Dsu;
use proconio::{input, marker::Chars};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                if i + 1 < h && s[i + 1][j] == '.' {
                    s[i + 1][j] = '*';
                }
                if i > 0 && s[i - 1][j] == '.' {
                    s[i - 1][j] = '*';
                }
                if j + 1 < w && s[i][j + 1] == '.' {
                    s[i][j + 1] = '*';
                }
                if j > 0 && s[i][j - 1] == '.' {
                    s[i][j - 1] = '*';
                }
            }
        }
    }

    let mut dsu = Dsu::new(h * w);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                if i + 1 < h && s[i + 1][j] == '.' {
                    dsu.merge(i * w + j, (i + 1) * w + j);
                }
                if j + 1 < w && s[i][j + 1] == '.' {
                    dsu.merge(i * w + j, i * w + j + 1);
                }
            }
        }
    }

    let mut map = BTreeMap::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let root = dsu.leader(i * w + j);
                *map.entry(root).or_insert(0) += 1;
            }
        }
    }

    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '*' {
                let mut set = BTreeSet::new();
                for k in 0..4 {
                    let ni = i as i32 + dx[k];
                    let nj = j as i32 + dy[k];
                    if ni < 0 || h as i32 <= ni || nj < 0 || w as i32 <= nj {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if s[ni][nj] == '.' {
                        let root = dsu.leader(ni * w + nj);
                        if set.contains(&root) {
                            continue;
                        }
                        set.insert(root);
                        *map.entry(root).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    println!("{}", map.iter().map(|(_, &v)| v).max().unwrap_or(1));
}
