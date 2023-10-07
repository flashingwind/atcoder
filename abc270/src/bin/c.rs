use petgraph::graph::node_index as n;
use petgraph::prelude::*;
use petgraph::visit::depth_first_search;
use petgraph::visit::{Control, DfsEvent};
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        nn: usize,
        x: Usize1,
        y: Usize1,
        es: [(Usize1,Usize1);nn-1],
    };

    let mut gr: UnGraph<(), ()> = Graph::new_undirected();
    let mut nodes = vec![];
    for _ in 0..nn {
        nodes.push(gr.add_node(()));
    }
    for e in es.iter() {
        gr.add_edge(nodes[e.0], nodes[e.1], ());
    }

    // record each predecessor, mapping node → node
    let mut predecessor = vec![NodeIndex::end(); gr.node_count()];
    let start = n(x);
    let goal = n(y);
    depth_first_search(&gr, Some(start), |event| {
        if let DfsEvent::TreeEdge(u, v) = event {
            predecessor[v.index()] = u;
            if v == goal {
                return Control::Break(v);
            }
        }
        Control::Continue
    });

    let mut next = goal;
    let mut path = vec![next];
    while next != start {
        let pred = predecessor[next.index()];
        path.push(pred);
        next = pred;
    }
    path.reverse();
    for (i, node) in path.iter().enumerate() {
        if i == 0 {
            print!("{}", node.index() + 1);
        } else {
            print!(" {}", node.index() + 1);
        }
    }
}
