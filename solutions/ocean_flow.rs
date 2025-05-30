use std::collections::HashSet;

type Ocean = HashSet<(usize,usize)>;


fn ocean_flow(topo:&[Vec<u8>])->Vec<(usize,usize)> {
    if topo.is_empty() || topo.len() == 0 { return vec![] };
    let m = topo.len();
    let n = topo[0].len();
    let mut pacific:Ocean = HashSet::with_capacity(m * n);
    let mut atlantic:Ocean = HashSet::with_capacity(m * n);
    
    for i in 0..m {
        dfs(i,0, &mut pacific, topo);
        dfs(i,n-1, &mut atlantic, topo);
    }
    for j in 0..n {
        dfs(0,j, &mut pacific, topo);
        dfs(m-1,j, &mut atlantic, topo);
    }
    pacific.intersection(&atlantic).cloned().collect::<Vec<(usize,usize)>>()
}

fn dfs(x:usize, y:usize, ocean:&mut Ocean, topo:&[Vec<u8>]) {
    if !ocean.insert((x,y)) { return };
    // simulated twos compliment
    let del= [(1,0), (0,1), (!0, 0), (0, !0)];
    let m = topo.len();
    let n = topo[0].len();

    for (dx,dy) in del {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);

        if nx < m && ny < n && topo[nx][ny] >= topo[x][y] {
            dfs(nx, ny, ocean, topo);
        }
    }
}

// // first pass
// fn ocean_flow(topo:&[Vec<u8>])->Vec<(usize,usize)> {
//     if topo.is_empty() || topo.len() == 0 { return vec![] };
//     let m = topo.len();
//     let n = topo[0].len();
//     let mut pacific:HashSet<(usize,usize)> = HashSet::with_capacity(m.max(n));
//     let mut atlantic:HashSet<(usize,usize)> = HashSet::with_capacity(m.max(n));
    
//     for i in 0..m {
//         flow_increase((i,0), &mut pacific, topo);
//         flow_decrease((i,n-1), &mut atlantic, topo);
//     }
//     for j in 0..n {
//         flow_increase((0,j), &mut pacific, topo);
//         flow_decrease((m-1,j), &mut atlantic, topo);
//     }
//     pacific.intersection(&atlantic).cloned().collect::<Vec<(usize,usize)>>()
// }

// fn flow_increase(p:(usize,usize), ocean:&mut HashSet<(usize,usize)>, topo:&[Vec<u8>]) {
//     ocean.insert(p);
//     let m = topo.len();
//     let n = topo[0].len();
//     if p.0+1 < m {
//         let p_xp = (p.0+1, p.1);
//         if topo[p.0][p.1] < topo[p_xp.0][p_xp.1] && !ocean.contains(&p_xp) {
//                 flow_increase(p_xp, ocean, topo)
//         }
//     }
//     if p.1 + 1 < n {
//         let p_yp = (p.0, p.1+1);
//         if topo[p.0][p.1] < topo[p_yp.0][p_yp.1] && !ocean.contains(&p_yp){
//             flow_increase(p_yp, ocean, topo)
//         }
//     }
// }

// fn flow_decrease(p:(usize,usize), ocean:&mut HashSet<(usize,usize)>, topo:&[Vec<u8>]) {
//     ocean.insert(p);
//     if p.0 > 0 {
//         let p_xm = (p.0-1, p.1);
//         if topo[p.0][p.1] < topo[p_xm.0][p_xm.1] && !ocean.contains(&p_xm) {
//                 flow_decrease(p_xm, ocean, topo)
//         }
//     }
//     if p.1 > 0  {
//         let p_ym = (p.0, p.1-1);
//         if topo[p.0][p.1] < topo[p_ym.0][p_ym.1] && !ocean.contains(&p_ym){
//             flow_decrease(p_ym, ocean, topo)
//         }
//     }
// }


fn main() {
    let a = ocean_flow(&[vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]]);

    println!("Result {:?}", a);
}
