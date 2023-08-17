use proconio::input;

fn main() {
    input! {
        n: usize,
        mut seg: [(u64,u64,u64);n],
    };
    let mut cnt: u64 = 0;
    //seg.sort_by(|a, b| a.1.cmp(&b.1));
    for (_, (t1, l1, r1)) in seg.iter_mut().enumerate() {
        if *t1 == 1 {
        } else if *t1 == 2 {
            *r1 -= 1;
        } else if *t1 == 3 {
            *l1 += 1;
        } else if *t1 == 4 {
            *r1 -= 1;
            *l1 += 1;
        }
    }

    for (i, (_t1, l1, r1)) in seg.iter().enumerate() {
        println!("seg[{i}]={:?}", seg[i]);
        let seg2 = seg.to_owned();
        println!("  sqg2: seg[{}..{})", (i + 1), n);
        let mut itr = seg2.iter();
        itr.nth(i);
        for (j, (_t2, l2, r2)) in itr.enumerate() {
            println!("    seg2 {j}={l2},{r2}");
            if *l2 <= *r1 && *l1 <= *r2 {
                cnt += 1;
                println!("      1 {l2} <= {r1} || {l1} <= {r2}");
            } else if *l1 <= *r2 && *l2 <= *r1 {
                cnt += 1;
                println!("      2 {l1} <= {r2} || {l2} <= {r1}");
            }
        }
    }
    println!("{}", cnt);
}
