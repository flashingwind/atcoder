use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut arr: [[char;h];w],
    };
    input! {
        q: usize,
    }
    for _ in 1..=q {
        input! {
            a: usize,
            b: usize,
        }
        let r1_i_start = 1usize;
        let r1_j_start = 1usize;
        let r1_i_end = a;
        let r1_j_end = b;

        let r2_i_start = 1;
        let r2_j_start = w - b;
        let r2_i_end = h  - a;
        let r2_j_end = w;

        let r3_i_start = h-a;
        let r3_j_start = 1;
        let r3_i_end = h;
        let r3_j_end = b;

        let r4_i_start = h - a;
        let r4_j_start = w - b;
        let r4_i_end = h;
        let r4_j_end = w;

        for i in r1_i_start..=r1_i_end {
            for j in r1_j_start..=r1_j_end {
                let tmp = arr[i-1][j-1];
                arr[r1_i_end-i-1][r1_j_end-j-1] = tmp;
            }
        }
        for i in r2_i_start..=r2_i_end {
            for j in r2_j_start..=r2_j_end {
                let tmp = arr[i-1][j-1];
                arr[r2_i_end-i-1][r2_j_end-j-1] = tmp;
            }
        }
        for i in r3_i_start..=r3_i_end {
            for j in r3_j_start..=r3_j_end {
                let tmp = arr[i-1][j-1];
                arr[r3_i_end-i-1][r3_j_end-j-1] = tmp;
            }
        }
        for i in r4_i_start..=r4_i_end {
            for j in r4_j_start..=r4_j_end {
                let tmp = arr[i-1][j-1];
                arr[r4_i_end-i-1][r4_j_end-j-1] = tmp;

            }
        }
    }
    for i in 1..=h{
        for j in 1..=w{
            print!("{}",arr[i-1][j-1]);
        }
        println!();
    }
}
