fn good_subarray(nums:&[usize], k:usize) -> usize {
    let n = nums.len();
    let mut max = nums[k];
    let mut h = nums[k];
    let mut l = k;
    let mut r = k;

    loop {
        let w = r - l + 1;
        max = max.max(h * w);
        if 0 < l && r < n - 1 {
            if nums[l-1] > nums[r+1] {
                h = h.min(nums[l-1]);
                l -= 1;
            } else {
                h = h.min(nums[r+1]);
                r += 1;
            }
        } else if 0 < l {
            h = h.min(nums[l-1]);
            l -= 1;
        } else if r < n-1 {
            h = h.min(nums[r+1]);
            r += 1;
        } else {
            break;
        }
    }
    max
}


fn histogram(nums:&[usize]) -> usize {
    let mut stack = vec![];
    let mut max = 0; 
    for i in 0..=nums.len() {
        let h =  if i == nums.len() { 0 } else {nums[i] };
        while let Some(&top ) = stack.last() {
            if h < nums[top] {
                let h = nums[top];
                let left = stack.last().map_or(0, |&idx| idx);
                let w = i - left;
                stack.pop();
                max = max.max(h * w);
            } else {
                break;
            }
        }
        stack.push(i);
    }
    max
}


fn main() {
    // println!(" partition {:?}", histogram(&[2,1,5,6,2,3]));
    println!(" max subarray {:?}", good_subarray(&[1,4,3,7,4,5], 3));
}
