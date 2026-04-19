use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc454/tasks/abc454_e
/// https://atcoder.jp/contests/abc454/editorial/19007
///
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   盤面の経路そのものより、各マスを点とみた格子上の全点1回通りの話にする
///   上下左右に1回動くと i+j の偶奇が必ず反転するので、市松模様の黒白だけ持てばよい
/// - それについて、何が分かれば答えになる？
///   まず Yes/No は黒白の個数条件だけで判定できる
///   Yes の場合は、穴 (A,B) を避けながら外側 2 行 / 2 列を順に削る構成ができればよい
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   個々の経路候補を全部考える必要はない
///   移動のたびに色が反転するので、通るマスの色は黒白黒白...と交互に決まる
///   (1,1),(N,N) はともに黒なので、使う黒マス数は白マス数より 1 多い必要がある
///   よって N が奇数なら不可、N が偶数でも除く (A,B) は白マス、つまり A+B が奇数でないと不可
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   まず n%2==1 または (a+b)%2==0 なら No
///   そうでなければ、先頭に付ける文字列と末尾に付ける文字列を持ち、
///   穴を含まない外側 2 行 / 2 列を蛇行で剥がして最後 2x2 に落とす
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut a: Usize1,
            mut b: Usize1,
        }

        if n % 2 == 1 || (a + b) % 2 == 0 {
            println!("No");
            continue;
        }

        // 上 2 行を左上から蛇行で食べ切って、次の段の左端へ降りる部品。
        let row_block = format!("{}D{}D", "R".repeat(n - 1), "L".repeat(n - 1));
        let row_block_rev = reversed_block(&row_block);
        // 高さ 2 の盤面で左 2 列を処理しながら、次の 2 列へ進む部品。
        let col_block = "DRUR".to_string();
        let col_block_rev = reversed_block(&col_block);

        // prefix: 今から先頭に積む経路
        // suffix: 最後に後ろへ付ける経路
        let mut prefix = vec![];
        let mut suffix = vec![];

        // 穴が上 2 行より下にある間は、上 2 行を丸ごと削れる。
        for _ in 0..(n / 2 - 1) {
            if a >= 2 {
                prefix.push(row_block.clone());
                a -= 2;
            } else {
                suffix.push(row_block_rev.clone());
            }
        }

        // 高さ 2 まで落としたら、今度は左 2 列 / 右 2 列を順に削る。
        for _ in 0..(n / 2 - 1) {
            if b >= 2 {
                prefix.push(col_block.clone());
                b -= 2;
            } else {
                suffix.push(col_block_rev.clone());
            }
        }

        // 最後は 2x2。穴の位置に応じて DR / RD のどちらかで終わる。
        if (a, b) == (0, 1) {
            prefix.push("DR".to_string());
        } else {
            prefix.push("RD".to_string());
        }

        let mut ans = String::new();
        for s in &prefix {
            ans.push_str(s);
        }
        for s in suffix.iter().rev() {
            ans.push_str(s);
        }

        println!("Yes");
        println!("{}", ans);
    }
}

fn reversed_block(s: &str) -> String {
    s.chars().rev().collect()
}
