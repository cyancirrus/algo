use std::cmp::Ordering;
// fn trapping_rain_water(nums:&[u32]) -> u32 {
//     let len = nums.len();
//     if len == 0 { return 0 };
//     let mut volume = 0;
//     let mut minus:Vec<(usize, u32)> = vec![];
//     let mut prev = 0;
//     for i in 0..len {
//         if prev < nums[i] {
//             if minus.len() != 0 {
//                 println!("minus {:?}", minus);
//                 // println!("val {:?}", nums[i]);
//                 let mut lower = minus.pop().unwrap();
//                 let mut d_len = minus.len();
//                 while d_len > 0 {
//                     if minus[d_len - 1].1 <=  nums[i] {
//                         println!("i, val ({}, {:?})", i, nums[i]);
//                         let val = minus.pop().unwrap();
//                         let leb = (i as u32 - val.0 as u32 - 1) * (val.1 - lower.1);
//                         println!("adding volume {:?}", leb);
//                         // println!("adding v,h: ({},{}), i,j:  ({}, {})", i,  val.0, leb);
//                         // println!("adding v,h: ({},{}), i,j:  ({}, {})", leb, (val.1 - lower.1), i, val.0);
//                         // println!("adding w,h: ({},{})", i  - val.0, (val.1 - lower.1));
//                         volume += (i as u32 - val.0 as u32 - 1) * (val.1 - lower.1);
//                         lower = val; 
//                         d_len -= 1;
//                     } else {
//                         break;
//                     }
//                 }
//                 println!("after remove minus {:?}", minus);
//             }
//             minus.push((i, nums[i]));
//         } else if prev > nums[i] {
//             minus.push((i, nums[i]));
//         }
//         prev = nums[i];
//     }
//     volume as u32
// }
//                         // let leb = (i as u32 - val.0 as u32) * (val.1 - lower.1);
//                         // println!("adding v,h: ({},{}), i,j:  ({}, {})", i,  val.0, leb);
                        // println!("adding v,h: ({},{}), i,j:  ({}, {})", leb, (val.1 - lower.1), i, val.0);

// fn trapping_rain_water(nums:&[u32]) -> u32 {
//     let len = nums.len();
//     if len == 0 { return 0 };
//     let mut volume = 0;
//     let mut minus:Vec<(usize, u32)> = vec![];
//     let mut prev = 0;
//     for i in 0..len {
//         if prev < nums[i] {
//             if minus.len() != 0 {
//                 // println!("val {:?}", nums[i]);
//                 let mut lower = minus.pop().unwrap();
//                 let mut d_len = minus.len();
//                 println!("minus {:?}", minus);
//                 while d_len > 0 {
//                     if minus[d_len - 1].1 <=  nums[i] {
//                         println!("i, val ({}, {:?})", i, nums[i]);
//                         let val = minus.pop().unwrap();
//                         let leb = (i as u32 - val.0 as u32 - 1) * (val.1 - lower.1);
//                         println!("adding volume {:?}", leb);
//                         // println!("adding v,h: ({},{}), i,j:  ({}, {})", i,  val.0, leb);
//                         // println!("adding v,h: ({},{}), i,j:  ({}, {})", leb, (val.1 - lower.1), i, val.0);
//                         println!("adding w,h: ({},{})", i  - val.0 - 1, (val.1 - lower.1));
//                         volume += (i as u32 - val.0 as u32 - 1) * (val.1 - lower.1);
//                         lower = val; 
//                         d_len -= 1;
//                     } else {
//                         break;
//                     }
//                 }
//                 println!("after remove minus {:?}", minus);
//             }
//             minus.push((i, nums[i]));
//         } else if prev > nums[i] {
//             minus.push((i, nums[i]));
//         }
//         prev = nums[i];
//     }
//     volume as u32
// }

fn trap_lebesque(nums:&[u32]) -> u32 {
    // might not work with repeated values
    let len = nums.len();
    if len == 0 { return 0 };
    let mut volume = 0;
    let mut minus:Vec<(usize, u32)> = vec![];
    let mut prev = 0;
    for i in 0..len {
        if prev < nums[i] {
            if minus.len() != 0 {
                let mut lower = minus.pop().unwrap();
                let mut d_len = minus.len();
                while d_len > 0 {
                    // pop and lebesque integrate each number is only popped once
                    // => O(n)
                    if minus[d_len - 1].1 <=  nums[i] {
                        let val = minus.pop().unwrap();
                        volume += (i as u32 - val.0 as u32 - 1) * (val.1 - lower.1);
                        lower = val; 
                        d_len -= 1;
                    } else {
                        let val = minus[d_len - 1];
                        // calculate any other needed raising for difference
                        volume += (i as u32 - val.0 as u32 - 1) * (nums[i] - lower.1);
                        break;
                    }
                }
            }
            minus.push((i, nums[i]));
        } else if prev > nums[i] {
            minus.push((i, nums[i]));
        }
        prev = nums[i];
    }
    volume as u32
}

fn trap_reinman(nums:&[u32]) -> u32 {
    let len = nums.len();
    let mut volume = 0;
    let mut left_max:Vec<u32> = vec![0; len];
    let mut right_max:Vec<u32> = vec![0; len];
    left_max[0] = nums[0];
    right_max[len - 1] = nums[len - 1];
    for i in 1..len {
        left_max[i] = left_max[i-1].max(nums[i]);
    }
    for i in (0..len - 1).rev() {
        right_max[i] = right_max[i+1].max(nums[i]);
    }
    for i in 0..len {
        volume += left_max[i].min(right_max[i]) - nums[i];
    }
    volume
}

fn trap_pointer(nums:&[u32]) -> u32 {
    if nums.len() == 0 {return 0};
    let (mut l, mut r) = (0, nums.len() - 1);
    let (mut max_l, mut max_r) = (0, 0);
    let mut volume = 0;

    // squeeze towards the middle from direction where lower 
    while l <= r {
        if nums[l] < nums[r] {
            if max_l <= nums[l] {
                max_l = nums[l];
            } else {
                // going to be contained b/c right > left
                volume += max_l - nums[l];
            }
            l+=1;
        } else {
            if max_r <= nums[r] {
                max_r = nums[r];
            } else {
                // going to be contained b/c left > right
                volume += max_r - nums[r];
            }
            r-=1;
        }
    }
    volume
}

fn trap_stack(nums:&[u32]) -> u32 {
    if nums.len() == 0 {return 0};
    let mut stack:Vec<usize> = vec![];
    let mut volume = 0;

    for (i, &h) in nums.iter().enumerate() {
        while let Some(&top) = stack.last() {
            if h > nums[top] {
                let bottom = stack.pop().unwrap();
                if let Some(&left) = stack.last() {
                    let distance = i - left - 1;
                    let bounded_height = h.min(nums[left]) - nums[bottom];
                    volume += distance as u32 * bounded_height;
                }
            } else {
                break;
            }
        }
        stack.push(i);
    }
    volume
}

fn trap_lebesque_dev(nums:&[u32]) -> u32 {
    // not yet working
    // if nums.len() == 0 {return 0};
    // let mut stack:Vec<usize> = vec![];
    // let mut volume = 0;

    // for (i, &h) in nums.iter().enumerate() {
    //     while let Some(&top) = stack.last() {
    //         if h > nums[top] {
    //             let bottom = stack.pop().unwrap();
    //             if let Some(&left) = stack.last() {
    //                 let distance = i - left - 1;
    //                 let bounded_height = h.min(nums[left]) - nums[bottom];
    //                 volume += distance as u32 * bounded_height;
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    //     if let Some(index) = stack.last() {
    //         if h == nums[*index] {
    //             stack.pop();
    //             stack.push(i);
    //         }
    //     }
    //     stack.push(i);
    // }
    // volume
    todo!()
}

fn histogram_first(nums:&[u32]) -> (usize,u32) {
    if nums.is_empty() { return (0,0)};
    let (mut l, mut r) = (1, nums.len() - 2);
    let (mut l_b, mut r_e) = (0, nums.len()-1);
    while l <= r {
        if nums[l] == nums[r] {
            l = r_e;
            r = l_b;
            
        } else if (l - l_b) < (r_e - r) {
            if nums[l] != nums[l-1] {
                l_b = l;
            }
            l+=1;
        } else {
            if nums[r] != nums[r+1] {
                r_e = r;
            }
            r-=1;
        }
    }
    if (l -l_b) > (r_e -r) {
        (l - l_b + 1, nums[l_b])
    } else {
        (r_e -r + 1, nums[r_e])
    }
}

fn histogram(nums:&[u32]) -> (usize,u32) {
    if nums.is_empty() { return (0,0) }
    let mut m_cnt = 1;
    let mut m_val = nums[0];
    let mut r_cnt = 0;
    let mut r_val = nums[0];
    
    for &c_val in nums {
        if m_val == c_val {
            m_cnt += 1;
        } else if r_val == c_val {
            r_cnt += 1;
        } else {
            if r_cnt > m_cnt {
                m_cnt = r_cnt;
                m_val = r_val;
            }
            r_val = c_val;
            r_cnt = 1;
        }
    }
    if r_cnt > m_cnt {
        (r_cnt, r_val)
    } else {
        (m_cnt, m_val)
    }
}


fn main() {
    assert_eq!(6, trap_lebesque(&[0,1,0,2,1,0,1,3,2,1,2,1]));
    assert_eq!(5, trap_lebesque(&[0,1,0,2,1,0,2,3,2,1,2,1]));
    assert_eq!(7, trap_lebesque(&[0,1,0,2,1,0,0,3,2,1,2,1]));
    assert_eq!(9, trap_lebesque(&[4,2,0,3,2,5]));
    // assert_eq!(6, trap_reinman(&[0,1,0,2,1,0,1,3,2,1,2,1]));
    // assert_eq!(5, trap_reinman(&[0,1,0,2,1,0,2,3,2,1,2,1]));
    // assert_eq!(7, trap_reinman(&[0,1,0,2,1,0,0,3,2,1,2,1]));
    // assert_eq!(9, trap_reinman(&[4,2,0,3,2,5]));
    // assert_eq!(6, trap_pointer(&[0,1,0,2,1,0,1,3,2,1,2,1]));
    // assert_eq!(5, trap_pointer(&[0,1,0,2,1,0,2,3,2,1,2,1]));
    // assert_eq!(7, trap_pointer(&[0,1,0,2,1,0,0,3,2,1,2,1]));
    // assert_eq!(9, trap_pointer(&[4,2,0,3,2,5]));
}
