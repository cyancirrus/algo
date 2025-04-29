// 450123

// 4570123
// * left side *
// if val > r and m < val and m > l, l-> m
// if val > r and m < val and m < l, u-> m
// if val > r and m > val, u-> m 

// * right side *
// if val < l and m < val, l -> m
// if val < l and m > val and m > u, l-> m
// if val < l and m > val and m < u, u -> m

fn binary_search_pivot(val:i32, nums:Vec<i32>) -> i32 {
    let len = nums.len() - 1;
    let (mut l, mut m, mut u) = (0, len/2, len);
    let mut _break = 0;
    loop {
        if nums[m] == val {
            return m as i32
        } else if u == l {
            break;
        } else if val > nums[len] {
            if nums[m] > val {
                u = m;
            } else if nums[m] < val && nums[m] > nums[0] {
                l = m + 1;
            } else if nums[m] < val && nums[m] < nums[0] {
                u = m;
            }
            m = ( l + u )/ 2;
        } else if val < nums[0] {
            if nums[m] < val {
                l = m + 1;
            } else if nums[m] > val && nums[m] > nums[len] {
                l = m + 1;
            } else if nums[m] > val && nums[m] < nums[len] {
                u = m;
            } else {
            }
            m = ( l + u )/ 2;
        }
    }
    -1
}


fn main() {
    let nums = vec![4,5,6,7,0,1,2];
    assert_eq!(0, binary_search_pivot(4, nums.clone()), "search value for 4");
    assert_eq!(1, binary_search_pivot(5, nums.clone()), "search value for 5");
    assert_eq!(2, binary_search_pivot(6, nums.clone()), "search value for 6");
    assert_eq!(3, binary_search_pivot(7, nums.clone()), "search value for 7");
    assert_eq!(4, binary_search_pivot(0, nums.clone()), "search value for 0");
    assert_eq!(5, binary_search_pivot(1, nums.clone()), "search value for 1");
    assert_eq!(6, binary_search_pivot(2, nums.clone()), "search value for 2");
    assert_eq!(-1, binary_search_pivot(-7, nums.clone()), "should not find value");
}
