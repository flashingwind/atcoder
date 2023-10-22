use petgraph::{algo::connected_components, prelude::*, stable_graph::IndexType};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h],
    };
    let d = vec![
        (-1_i64, -1_i64),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut n: Vec<Vec<Option<NodeIndex>>> = vec![vec![None; w]; h];
    let mut g: Graph<(), (), Undirected> = UnGraph::new_undirected();
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '#' {
                if let Some(_) = n[y][x] {
                } else {
                    let a = g.add_node(());
                    n[y][x] = Some(a);
                }
                let a = n[y][x].unwrap();
                for dydx in d.iter() {
                    let y2 = dydx.0 + y as i64;
                    let x2 = dydx.1 + x as i64;
                    if 0 <= y2 && 0 <= x2 && y2 < h as i64 && x2 < w as i64 {
                        if s[y2 as usize][x2 as usize] == '#' {
                            if let Some(_) = n[y2 as usize][x2 as usize] {
                            } else {
                                let b = g.add_node(());
                                n[y2 as usize][x2 as usize] = Some(b);
                            }
                            let b = n[y2 as usize][x2 as usize].unwrap();
                            g.add_edge(a, b, ());
                        }
                    }
                }
            }
        }
    }
    // for i in 0..h {
    //     println!(
    //         "{:?}",
    //         n[i].iter().map(|v| {
    //             if let Some(a) = v {
    //                 a.index() as usize
    //             } else {
    //                 0usize
    //             }
    //         })
    //     );
    // }
    println!("{}", connected_components(&g));
}
