use proconio::input;

/// https://atcoder.jp/contests/abc403/tasks/abc403_e
/// https://atcoder.jp/contests/abc403/editorial/12825
/// 本問題は，トライ木 (Trie) の練習問題です．
/// トライ木は，文字列のリストを根付き木として表現するデータ構造であり，
/// 木の各頂点に適切な情報を持たせることで，接頭辞に関する処理を効率的に行うことができます．
fn main() {
    input! {
        q: usize,
        ts: [(usize, String); q],
    }

    let mut trie = Trie::new();
    let mut y_cnt = 0;
    for (i, (t, s)) in ts.iter().enumerate() {
        trie.add(s, i, *t == 2);
        if *t == 2 {
            y_cnt += 1;
        }
        println!("{}", y_cnt - trie.z_set.len());
    }
}

use std::collections::HashSet;

struct Node {
    child: [Option<usize>; 26],
    f: bool,
    z_set: HashSet<usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            child: [None; 26],
            f: false,
            z_set: HashSet::new(),
        }
    }
}

struct Trie {
    nodes: Vec<Node>,
    z_set: HashSet<usize>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
            z_set: HashSet::new(),
        }
    }

    fn add(&mut self, s: &str, i: usize, is_y: bool) {
        let mut k = 0;
        for b in s.bytes() {
            let c = (b - b'a') as usize;
            if self.nodes[k].child[c].is_none() {
                self.nodes[k].child[c] = Some(self.nodes.len());
                self.nodes.push(Node::new());
            }
            if is_y {
                self.nodes[k].z_set.insert(i);
                if self.nodes[k].f {
                    self.z_set.insert(i);
                }
            }
            k = self.nodes[k].child[c].unwrap();
        }
        if is_y {
            self.nodes[k].z_set.insert(i);
            if self.nodes[k].f {
                self.z_set.insert(i);
            }
        } else {
            self.nodes[k].f = true;
            for &j in &self.nodes[k].z_set {
                self.z_set.insert(j);
            }
            self.nodes[k].z_set.clear();
        }
    }
}
