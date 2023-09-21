use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize;m];n],
    };
    let ai0 = (b[0][0] - 1) / 7 * 7 + 1;
    // println!("ai0={ai0}");
    let j = (b[0][0] - 1) % 7;
    for di in 0..n {
        for dj in 0..m {
            //j=0..=6
            // println!(
            //     "{di},{dj} {} {}=={}",
            //     j + dj,
            //     b[di][dj],
            //     (b[0][0] + di * 7 + dj)
            // );
            if 7 <= j + dj || b[di][dj] != (ai0 + di * 7 + j + dj) {
                println!("No");
                return;
            }
        }
        // println!();
    }
    println!("Yes");
    // b[i][j]=(i−1)×7+j
    // b[i][j+1]=b[i][j]+1
    // b[i+1][j]=b[i][j]+7
}
