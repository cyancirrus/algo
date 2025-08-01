fn find_the_duplicate(nums:&mut [usize]) -> usize {
    for i in 0..nums.len() {
        let j = nums[i];
        if j-1 != i && nums[j-1] == j {
            return j;
        }
        nums.swap(j-1, i);
    }
    0
}

// idea can i navigate through the datastructure with indices?
// there should be one index i haven't seen
//
// [1,2,3,4,5] <- visit all indices
// [5,4,2,1,3] <- visits all indices
// -> so if we go to like idx % len where do we stop
//
// [1,3,4,2,2]
// 1 ->3 -> 2 -> 3 -> 2 (cycle detected) size 2
// (we could store the previous things) but how much we need
// 
// hmmm this doesn't 
// [2, 3, 1, 2]
//
// [3,1,3,4,2]
// (3 -> 4 -> 2) -> (3 -> 4 -> 2) 
// cycle size 3
//
// cycles aren't determinitive consider
// [1, 4, 2, 3] (1,4) is a cycle
//
// [5, 1, 2, 3, 4] <- stable 
// [5, 1, 2, 3, 3] <- contains loop stable 
// could do like the rabit strategy
// 0, idx = n-2, n-2 
// lets say that i could have memory i could store a frequency and just chose 2
//  i could also do it by for 0..n search for n
//
//  what happens if i did a sum n(n+1) / 2 so 4 ~ 10
//  // notice there's also one missing
//
//  [1, 2, 3, 3]  (9)
//  [2, 2, 1, 4]  (9)
//  [4, 2, 1, 4]  (11)
//  [1, 1, 2, 4]  (7) 
//  [1, 1, 2, 4]  (8) 
//  [1, 1, 3, 4]  (9) 
// sums don't work
// 
// i could use the rabbit and the hair strategy

fn find_duplicate_no_mutate(nums:&[usize]) -> usize {
    let mut hare = nums[0];
    let mut tort = nums[0];

    loop {
        hare = nums[nums[hare]] ;
        tort = nums[tort] ;
        if hare == tort {
            break
        }
    }
    hare = nums[0];
    loop {
        if hare == tort {
            return hare
        }
        hare = nums[hare];
        tort = nums[tort];
    }
}

// fn find_the_duplicate_no_mutate(nums:&[usize]) -> usize {
//     let mut hare = 0;
//     let mut tort = 1;
//     let n = nums.len();
//     let mut i = 0;

//     loop {
//         println!("i {}", i);
//         if nums[hare] == nums[tort] && hare != tort {
//             break
//         }
//         hare = (hare + 2) % n;
//         tort = (tort + 1) % n;
//         i+=1;
//     }
//     hare = 0;
//     loop {
//         println!("Hare tort ({}, {})", hare, tort);
//         hare += 1;
//         tort += 1;
//         if hare == tort {
//             return nums[hare]
//         }
//     }
// }


fn main() {
    assert_eq!(2, find_the_duplicate(&mut [1,3,4,2,2]));
    assert_eq!(4, find_the_duplicate(&mut [1,4,4,3,2]));
    assert_eq!(3, find_the_duplicate(&mut [3,1,3,4,2]));
    assert_eq!(2, find_duplicate_no_mutate(&mut [1,3,2,2]));
    // this returns 5
    assert_eq!(4, find_duplicate_no_mutate(&mut [1,4,4,3,2,0]));
    assert_eq!(3, find_duplicate_no_mutate(&mut [3,1,3,4,2,0]));
    assert_eq!(3, find_duplicate_no_mutate(&mut [3,1,3,4,2,0]));
    assert_eq!(3, find_duplicate_no_mutate(&mut [3,1,3,4,5,0]));
    assert_eq!(3, find_duplicate_no_mutate(&mut [3,5,3,4,2,0]));

}
