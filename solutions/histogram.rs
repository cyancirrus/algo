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
    println!(" partition {:?}", histogram(&[2,1,5,6,2,3]));
}
