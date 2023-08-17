use petgraph::algo::bellman_ford;
use petgraph::prelude::Graph;
use proconio::input;

fn main() {
    input! {
        n: usize,
        _k: usize,
        a: [[u32;n];n],
        q: usize,
    };
    let mut graph: Graph<(), f32> = Graph::new();
    let mut nodes = Vec::new();
    for _ in 0..n {
        nodes.push(graph.add_node(()));
    }
    let mut es = Vec::new();
    for (i, ni) in nodes.iter().enumerate() {
        for (j, nj) in nodes.iter().enumerate() {
            if a[i][j] == 1 {
                es.push((*ni, *nj, 1.0));
                // println!("{i}={ni:?},{j}={nj:?}");
            }
        }
    }
    graph.extend_with_edges(es.iter());

    // let mut res = HashMap::new();
    for _ in 0..q {
        input! {
            mut s_raw: usize,
            mut t_raw: usize,
        };
        let mut s = s_raw - 1;
        let mut t = t_raw - 1;
        s %= n;
        t %= n;
        if s == t {
            let n = graph.add_node(());
            let mut es = Vec::new();
            for (j, nj) in nodes.iter().enumerate() {
                if a[s][j] == 1 {
                    es.push((n, *nj, 1.0));
                    // println!("{i}={ni:?},{j}={nj:?}");
                }
            }
            graph.extend_with_edges(es.iter());
            if let Ok(path) = bellman_ford(&graph, n) {
                if path.0[t] != std::f32::INFINITY {
                    println!("{}", path.0[t] as i32);
                } else {
                    println!("-1");
                }
            } else {
                println!("-1");
            }
        } else {
            // println!("{}, {}", s, t);
            // if let Some(m) = res
            //     .entry(nodes[s])
            //     .or_insert(dijkstra(&graph, nodes[s], None, |_| 1))
            //     .get(&nodes[t])
            // {
            //     println!("{}", m);
            // } else {
            //     println!("-1");
            // }
            if let Ok(path) = bellman_ford(&graph, nodes[s]) {
                if path.0[t] != std::f32::INFINITY {
                    println!("{}", path.0[t] as i32);
                } else {
                    println!("-1");
                }
            } else {
                println!("-1");
            }
        }
    }
}
