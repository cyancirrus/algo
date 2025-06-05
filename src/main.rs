use std::collections::BTreeSet;
// [4, 2, 5, 0, 3, 1]
// check if there is a parter in both options
//
// if i swap 4 w (3 V 1)
// if i swap 2 w (5 V 0)
//
// => swap with the partner
//  else skip 



// values [4, 2, 5, 0, 3, 1]
// idx is now val
// postion [3, 5, 1, 4, 0, 2]
//
// values [0, 3, 1, 2, 5, 4]
// postion [0, 2, 3, 1, 5, 4]
// > should swap (3, 2)
//
// > p = 0;
//  v_1 is located at p_2
// > i could swap 3 with (1 V 2)
// -> position[idx + 1]
//
// there are two decisions 
// -> exchange p0 with the parter of p1
// -> exhance p1 with the partner of p0
//
// (a, c), (d, f), (b, e)
//  if i swap a and d

// values [5, 0, 1, 4, 2, 3]
// postion [1, 2, 4, 5, 3, 0]
//
// should swap 0 and 1
// 0 is in the current thing
// 1 is located in second
// ->
// inspect the second set
// does



fn couples_swap(couples:&mut [usize])-> &[usize] {
    let n =  couples.len();
    let mut position = vec![0;n];
    let mut unsolved = BTreeSet::new();
    // let mut solved = 0;

    for idx in (0..n).step_by(2)  {
        if couples[idx] != couples[idx + 1] {
            unsolved.insert(idx);
        }
        position[couples[idx]] = idx;
        position[couples[idx + 1]] = idx + 1;
    }
    let mut count = unsolved.len();
    // while count != 0 {
    //     if let Some(idx) = unsolved.pop_first() {
    //         println!("idx {}", idx);
    //         // values
    //         let v_a = couples[idx]; // 
    //         let v_b = couples[idx + 1]; 
    //         // partner position
    //         let q_a = position[couples[idx] ^ 1];
    //         let q_b = position[couples[idx+1] ^ 1];

    //         println!("(p_a, p_b), :: (v_a, v_b)");
    //         println!(" :: ({}, {})",  v_a, v_b);
    //         println!("(q_a, q_b) :: (c_a, c_b)  ");
    //         println!("({}, {}) :: ({}, {})", q_a, q_b, couples[idx] ^ 1, couples[idx+1] ^ 1);
    //         if q_a ==  q_b ^ 1 {
    //             println!("hello world");
    //             couples.swap(idx + 1, q_a);
    //             unsolved.remove(&(q_a % 2));
    //             count -= 1;
    //         } else {
    //             println!("------");
    //             println!("Unsolved {:?}", unsolved);
    //             unsolved.insert(idx);
    //             println!("Unsolved {:?}", unsolved);
    //             println!("------");
    //         }
    //         count -= 1;
    //     }
    // }
    // println!("Unsolved {:?}", unsolved);
    println!("couples {:?}", couples);
    println!("Position: {:?}", position);
    println!("-------------------------------");
    while let Some(idx) = unsolved.pop_last() {
        let q_a = position[couples[idx] ^ 1];
        let q_b = position[couples[idx+1] ^ 1];
        let p_qa = position[idx + 1];
        let p_qb = position[couples[idx + 1]];
        couples.swap(idx + 1, q_a);
        position.swap(p_qa, p_qb);
        if q_a == q_b ^ 1 {
            unsolved.remove(&(q_a % 2));
        }
    }
    couples
}

// fn couple_swap(


// fn couples_swap(couples:&mut [usize])-> &[usize] {
//     let (mut i, mut j) = (0, 0);
//     loop {
//         if i != couples[i] { break; }
//         i+=1;
//     }
//     for i in 0..couples.len() {
//         // if i == couples[i] { break; }
//         couples.swap(i, couples[i])
//     }
//     couples
// }

fn main() {
    // println!("What does this do {:?}", couples_swap(&mut [2,3,0,1]));
    println!("What does this do {:?}", couples_swap(&mut [4,2,5,0,3,1]));
    // println!("What does this do {:?}", couples_swap(&mut [0,2,1,3]));
    // assert_eq!(0, couples_swap(&mut [0,1,2,3]));
    // assert_eq!(0, couples_swap(&mut [2,3,0,1]));
    // assert_eq!(0, couples_swap(&mut [2,3,0,1]));
    // assert_eq!(1, couples_swap(&mut [2,0,3,1]));
    // assert_eq!(2, couples_swap(&mut [4,2,5,0,3,1]));
}
