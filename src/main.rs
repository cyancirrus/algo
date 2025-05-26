use std::mem;
// use std::time::Instant;
// use std::hint::black_box;


// if l < r 
// => h = min(l, r) = l
// if i shift r to r - 1
// => h = min(l, r-1) = l
// => shifting the higher of the two will always be less ie

// if l < r 
// min(l, r) * w < min(l, r-1) * (w-1)
// by symettry same for r > l 
// => we find the max by comparing volume at each possible value
// which is not strictly dominated by this inequality


fn container_most_water(nums:&[usize]) -> usize {
    // cleaning up the function 
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut volume = 0;
    while l != r {
        let h = nums[l].min(nums[r]);
        let w = r - l;
        volume = volume.max(h * w);
        if nums[l] < nums[r] {
            l+=1;
        } else {
            r-=1;
        }
    }
    volume
}


// fn container_most_water(nums:&[usize]) -> usize {
//     // storing the lmax and rmax is redundant
//     if nums.is_empty() { return 0 };
//     let mut l = 0;
//     let mut r = nums.len() - 1;
//     let mut volume = nums[l].min(nums[r]) * r;
//     while l != r {
//         if nums[l] < nums[r] {
//             l+=1;
//             if nums[l] > nums[l-1] {
//                 volume = volume.max(nums[l].min(nums[r]) * (r - l));
//             }
//         } else {
//             r-=1;
//             if nums[r] > nums[r+1] {
//                 volume = volume.max(nums[l].min(nums[r]) * (r - l));
//             }
//         }
//     }
//     volume
// }


// fn container_most_water(nums:&[usize]) -> usize {
//     // maximize width * height for two values
//     if nums.is_empty() { return 0 };
//     let mut l = 0;
//     let mut r = nums.len() - 1;
//     let mut lmax = nums[l];
//     let mut rmax = nums[r];
//     let mut volume = lmax.min(rmax) * r;
//     while l != r {
//         if lmax < rmax {
//             l+=1;
//             if nums[l] > lmax {
//                 lmax = nums[l];
//                 volume = volume.max(lmax.min(rmax) * (r - l));
//             }
//         } else {
//             r-=1;
//             if nums[r] > rmax {
//                 rmax = nums[r];
//                 volume = volume.max(lmax.min(rmax) * (r - l));
//             }
//         }
//     }
//     volume
// }




fn main() {
    assert_eq!(49, container_most_water(&[1,8,6,2,5,4,8,3,7]));
    assert_eq!(1, container_most_water(&[1,1]));
    assert_eq!(0, container_most_water(&[1]));
}
