use std::cmp::Ordering;

fn target_idx(nums:&[usize], target:usize) -> Vec<usize> {
    let mut lidx = 0;
    let mut cnt = 0;
    for n in nums {
        match target.cmp(n) {
            Ordering::Equal => { cnt += 1 },
            Ordering::Greater => { lidx += 1 },
            _ => {},
        }
    }
    (lidx..lidx+cnt).collect()
}


fn target_indices(nums:&[usize], target:usize) -> Vec<usize> {
    let mut lidx = 0;
    let mut cnt = 0;
    for &n in nums {
        if n < target {
            lidx+=1;
        } else if n == target {
            cnt +=1;
        }
    }
    (lidx..lidx+cnt).collect()
}

fn main() {
    println!("target indicies {:?}", target_indices(&[1,2,5,2,3], 2));
    println!("target indicies {:?}", target_indices(&[1,2,5,2,3], 3));
    println!("target indicies {:?}", target_indices(&[1,2,5,2,3], 0));

}
