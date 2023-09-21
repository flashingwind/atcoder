use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        gs: [(Usize1,Usize1);m],
    };
    let mut group_ids = vec![0; n];
    for (i, gid) in group_ids.iter_mut().enumerate() {
        *gid = i;
    }
    //println!("group_ids name={:?}", group_ids);
    let mut is_all_marked = false;
    while !is_all_marked {
        is_all_marked = true;
        for (u, v) in gs.iter() {
            if group_ids[*u] < group_ids[*v] {
                group_ids[*v] = group_ids[*u];
                is_all_marked = false;
                //println!(" {}>{} {:?}",group_ids[*u], group_ids[*v], group_ids);
            } else if group_ids[*v] < group_ids[*u] {
                group_ids[*u] = group_ids[*v];
                is_all_marked = false;
                //println!(" {}>{} {:?}",group_ids[*u], group_ids[*v], group_ids);
            }
        }
    }
    //println!("group_ids name={:?}", group_ids);
    let mut cnt = 1;
    let mut gid_stack = vec![0; 1];
    for gid in group_ids.iter_mut() {
        let mut is_diff = true;
        for stack_item in gid_stack.iter() {
            if *stack_item == *gid {
                is_diff = false;
                break;
            }
        }
        if is_diff {
            gid_stack.push(*gid);
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
