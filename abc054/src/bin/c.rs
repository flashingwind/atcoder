use petgraph::{csr::NodeIndex, prelude::UnGraph, Graph, Undirected};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1);m],
    };
    let mut g = UnGraph::<(), ()>::new_undirected();
    let mut nodes = vec![];
    for _ in 0..n {
        nodes.push(g.add_node(()));
    }
    for i in 0..m {
        g.add_edge(nodes[ab[i].0], nodes[ab[i].1], ());
    }

    let mut is_visited = vec![false; n];
    is_visited[0] = true;
    println!("{}", dfs(&mut g, 0, &mut is_visited));
}

fn dfs(g: &mut Graph<(), (), Undirected>, start: NodeIndex, is_visited: &mut Vec<bool>) -> usize {
    // println!("{}: {:?}", start, is_visited);
    if is_visited.iter().fold(true, |a, &x| a && x) {
        return 1;
    }
    let mut cnt = 0;
    let binding = g.clone();
    let ns = binding.neighbors((start as NodeIndex).into());
    for n in ns {
        if is_visited[n.index()] {
            continue;
        }
        is_visited[n.index()] = true;
        cnt += dfs(g, (n.index()) as u32, is_visited);
        is_visited[n.index()] = false;
    }
    return cnt;
}
