use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t_recv :Chars,
        t: [Chars;n],
    };
    let mut ans = vec![];
    for i in 0..n {
        if t_recv.len() == t[i].len() {
            // 1. T′は、Tと等しい。
            // 4  T′は、T のある 1文字を別の英小文字に変更して得られる文字列である。

            let mut err_cnt = 0;
            for j in 0..t_recv.len() {
                if t_recv[j] != t[i][j] {
                    // println!("{}==: jr{j}:{} j{j}:{}", i + 1, t_recv[j], t[i][j]);
                    err_cnt += 1;
                    if 2 <= err_cnt {
                        break;
                    }
                }
            }
            if err_cnt <= 1 {
                // println!("{}: == ", i + 1);
                ans.push(i);
            }
        } else if t_recv.len() == t[i].len() + 1 {
            // 2. T′は、T のいずれか 1 つの位置(先頭と末尾も含む)に英小文字を 1 つ挿入して得られる文字列である。
            let mut err_cnt = 0;
            let mut j = 0;
            for j_recv in 0..t_recv.len() {
                if t_recv[j_recv] != t[i][j] {
                    // println!(
                    //     "{}add: jr{j_recv}:{} j{j}!={}",
                    //     i + 1,
                    //     t_recv[j_recv],
                    //     t[i][j]
                    // );
                    // println!(
                    //     "{}add: jr{}:{} j{}=={}",
                    //     i + 1,
                    //     j_recv,
                    //     t_recv[j_recv + 1],
                    //     j,
                    //     t[i][j]
                    // );
                    err_cnt += 1;
                    if 2 <= err_cnt {
                        break;
                    }
                } else {
                    j += 1;
                    j = j.min(t[i].len() - 1);
                }
                if t[i].len() <= j {
                    break;
                }
            }
            if err_cnt <= 1 {
                // println!("{}: add ", i + 1);
                ans.push(i);
            }
        } else if t_recv.len() == t[i].len() - 1 {
            // 3. T′は、T からある 1 文字を削除して得られる文字列である。
            let mut err_cnt = 0;
            let mut j_recv = 0;
            for j in 0..t[i].len() {
                // println!(
                //     "{}del: jr{j_recv}:{} j{j} - {}",
                //     i + 1,
                //     t_recv[j_recv],
                //     t[i][j]
                // );
                if t_recv[j_recv] != t[i][j] {
                    // println!(
                    //     "{}del: jr{j_recv}:{} j{}=={}",
                    //     i + 1,
                    //     t_recv[j_recv],
                    //     j + 1,
                    //     t[i][j + 1]
                    // );
                    err_cnt += 1;
                    if 2 <= err_cnt {
                        break;
                    }
                } else {
                    j_recv += 1;
                    j_recv = j_recv.min(t_recv.len() - 1);
                }
            }
            if err_cnt <= 1 {
                // println!("{}: del", i + 1);
                ans.push(i);
            }
        }
    }
    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|v| (v + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
