use std::ops::Add;
use std::iter::Sum;
use std::hash::Hash;
use std::fmt::Debug;
use std::collections::HashMap;

// fn find_candidates<T> (target:T, candidates:&[T]) -> Vec<Vec<T>> 
//     where T:Add<Output=T> + PartialOrd + Copy + Clone + Sum,
// {
//     // First version just to start to see dynamics
//     let mut running_sums: Vec<Vec<T>> = vec![vec![]];
//     let mut result: Vec<Vec<T>> = Vec::new();

//     for value in candidates {
//         let mut new_sums:Vec<Vec<T>> = vec![];
//         for sums in &running_sums {
//             let mut new_sum = sums.clone();
//             new_sum.push(value.clone());
//             let cur_sum = new_sum.iter().cloned().sum::<T>();
//             if cur_sum  < target {
//                 new_sums.push(new_sum)
//             } else if cur_sum == target {
//                 result.push(new_sum);
//             }
//         }
//         running_sums.extend(new_sums);
//     }
//     result
// }

fn find_candidates<T> (target:T, candidates:&[T]) -> Vec<Vec<T>> 
    where T:Add<Output=T> + PartialOrd + Copy + Clone + Sum + Eq + Hash + Debug,
{
    let mut result: Vec<Vec<T>> = Vec::with_capacity(candidates.len());
    let mut rsum_map: HashMap<T, Vec<Vec<T>>> = HashMap::with_capacity(candidates.len());
    for candy in candidates {
        let mut new_coll = HashMap::new();
        if *candy < target {
            new_coll.insert(*candy, vec![vec![*candy]]);
        }

        for (key, value) in rsum_map.iter() {
            let new_key = *candy + *key;
            if new_key < target {
                let value = value.iter().
                    map(|vec| {
                        let mut new_vec = vec.clone();
                        new_vec.push(*candy);
                        new_vec
                    }).collect();
                new_coll.insert(new_key, value);
            } else if new_key == target {
                let inserts = value.iter().
                    map(|vec| {
                        let mut new_vec = vec.clone();
                        new_vec.push(*candy);
                        new_vec
                    }).collect::<Vec<_>>();
                result.extend(inserts);
            }
        }
        println!("{:?}", new_coll);
        rsum_map.extend(new_coll);
    }
    result
}


fn main() {
    let candidates = vec![2, 3, 5, 6, 1];
    let target = 8;

    let test = find_candidates(target, &candidates);
    println!("Test {:?}", test);
}
