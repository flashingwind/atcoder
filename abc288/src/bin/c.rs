use proconio::input;
use std::{collections::BTreeMap, vec};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize,usize);m],
    };

    for (a, b) in ab.iter_mut() {
        if *b < *a {
            std::mem::swap(&mut (*a), &mut (*b));
        }
    }
    ab.sort();
    // println!("ab={:?}", ab);
    let mut node_grp = vec![0; n + 1];
    // init.
    for (a, b) in ab.iter() {
        node_grp[*a] = *a;
        node_grp[*b] = *a;
    }
    // union nodes
    {
        let mut is_changed = true;
        while is_changed {
            is_changed = false;
            for (a, b) in ab.iter() {
                if node_grp[*a] < node_grp[*b] {
                    node_grp[*b] = node_grp[*a];
                    is_changed = true;
                } else if node_grp[*a] > node_grp[*b] {
                    node_grp[*a] = node_grp[*b];
                    is_changed = true;
                }
            }
        }
    }
    // 補完
    for (nid, gid) in node_grp.iter_mut().enumerate() {
        if *gid == 0 {
            *gid = nid;
        }
    }
    // println!("{:?}", node_grp);
    let mut num_nodes_of_g: BTreeMap<usize, usize> = BTreeMap::new();

    for gid in node_grp.iter().skip(1) {
        *num_nodes_of_g.entry(*gid).or_insert(0) += 1;
    }
    // println!("num_nodes_of_g={:?}", num_nodes_of_g);
    let mut num_edges_of_g: BTreeMap<usize, usize> = BTreeMap::new();
    for (a, _) in ab {
        *num_edges_of_g.entry(node_grp[a]).or_insert(0) += 1;
    }
    // println!("num_edges_of_g={:?}", num_edges_of_g);
    let mut sum = 0;
    for (gid, n_cnt) in num_nodes_of_g {
        if 3 <= n_cnt {
            let e_cnt = *num_edges_of_g.entry(gid).or_insert(0);
            if (n_cnt - 1) <= e_cnt {
                sum += e_cnt - (n_cnt - 1);
            }
        }
    }
    println!("{}", sum);
}
