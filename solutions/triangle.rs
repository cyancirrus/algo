// fn triangle(mut defn:Vec<Vec<i32>>) -> i32 {
//     if defn.is_empty() {return 0};
//     let mut path = defn.pop().unwrap();

//     while let Some(cur) = defn.pop() {
//         for i in 0..cur.len() {
//             path[i] = cur[i] + path[i].min(path[i+1]);
//         }
//     }
//     path[0]
// }

fn triangle(defn:&mut [Vec<i32>]) -> i32 {
    if defn.is_empty() {return 0};
    for r in (0..defn.len()-1).rev() {
        for i in 0..defn[r].len() {
            defn[r][i] = defn[r+1][i] + defn[r+1][i].min(defn[r+1][i+1]);
        }
    }
    defn[0][0]
}
fn main() {
    println!("Test {:?}", triangle(&mut [vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]));
}
