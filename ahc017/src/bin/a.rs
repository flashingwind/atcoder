use proconio::input;
use rand::Rng;

// fn main() {
//     input! {
//         num_nodes: usize,
//         num_edges: usize,
//     };
//     let mut m = vec![vec![0f32 / num_edges as f32; num_edges]; num_nodes];
//     println!("{:?}", m);
//     println!("{:?}", m[0]);
//     println!("{:?}", m[1]);
//     println!("{:?}", m[2]);
//     println!("{:?}", m[9][1]);
// }
fn main() {
    input! {
        num_nodes: usize,
        num_edges: usize,
        days: usize,
        num_const_per_day: usize,
        //(ui,vi,wi)はi番目の辺が頂点uiとviを重みwiで結んでいることを表す
        mut edges: [(usize,usize,u32);num_edges],
        mut _xy: [(usize,usize);num_nodes],
    };
    //edges.sort_by(|a, b| a.0.cmp(&b.0));
    // [[x;3]4] => [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]]
    // m[0]〜m[2][3]
    let mut m = vec![vec![0f32 / num_edges as f32; num_edges]; num_nodes];
    let mut p = vec![vec![1f32 / num_edges as f32; num_edges]; num_edges];
    let mut v = vec![1f32 / num_edges as f32; num_edges];
    for (i_e, (n1, n2, w)) in edges.iter().enumerate() {
        println!("{},{}/{},w={}", i_e, *n1 - 1, *n2 - 1, *w);
        m[i_e][*n1 - 1] = 1.0 / (*w as f32);
        m[i_e][*n2 - 1] = 1.0 / (*w as f32);
    }
    println!("P = E x E^t, 正規化");
    for j in 0..num_edges {
        let mut sum = 0.0;
        for k in 0..num_edges {
            for i in 0..num_nodes {
                p[j][k] += m[i][j] * m[i][k];
            }
            sum += p[j][k];
        }
        for k in 0..num_edges {
            p[j][k] = 0.8 * (p[j][k] / sum) + 0.2;
        }
    }
    for _t in 1..=3 {
        let mut v_tmp = vec![1f32 / num_edges as f32; num_edges];
        let mut sum = 0.0;
        for j in 0..num_edges {
            v_tmp[j] = 0.0;
            for k in 0..num_edges {
                v_tmp[j] += v[j] * p[j][k];
            }
            sum += v_tmp[j];
        }
        for j in 0..num_edges {
            v[j] = v_tmp[j] / sum;
        }
        println!("v={:?} sum(v)={}", v, sum);
    }

    let mut e_constday = vec![0usize; num_edges];
    {
        let mut rng = rand::thread_rng();
        // let mut cnt = 0;
        let avg_limit = (num_edges as f32 / num_const_per_day as f32).ceil().ceil() as usize;
        for day in 1..=days {
            println!("day:{}/{}", day, days);
            let mut cnt_today = 0;
            while cnt_today <= num_const_per_day && cnt_today <= avg_limit {
                let mut sum_v = 0.0;
                let y: f32 = rng.gen(); // generates a float between 0 and 1
                println!("y={y}");
                for j in 0..num_edges {
                    sum_v += v[j];
                    println!("if const: sum_v={sum_v}");
                    if sum_v <= y && e_constday[j] == 0 {
                        e_constday[j] = day;
                        cnt_today += 1;
                        println!("const: {j}");
                    }
                }
                // cnt += cnt_today;
            }
        }
    }
    if e_constday[0] != 0 {
        print!("{}", e_constday[0]);
    }
    for j in 1..num_edges {
        if e_constday[j] != 0 {
            print!(" {}", e_constday[j]);
        }
    }
}
// fn output(schedule_day_edge: Vec<(usize, usize)>) {
//     let s = schedule_day_edge.iter();
//     for (day, edge) in s {
//         print!(" {}", day);
//     }
// }
