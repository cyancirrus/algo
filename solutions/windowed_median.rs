use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;
use std::fmt::Debug;
use num_traits::ToPrimitive;

#[derive(Eq, PartialEq, Debug)]
struct MinNode<T> {
    elem:T
}

#[derive(Eq, PartialEq, Debug)]
struct MaxNode<T> {
    elem:T
}

impl<T> Ord for MaxNode<T> 
where T:Eq + PartialEq + Ord
{
    fn cmp(&self, other:&Self) -> Ordering {
        self.elem.cmp(&other.elem)
    }
}

impl<T> PartialOrd for MaxNode<T>
where T:Eq + PartialEq + Ord
{
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <T> Ord for MinNode<T> 
where T: Eq + PartialEq + Ord
{
    fn cmp(&self, other:&Self) -> Ordering {
        other.elem.cmp(&self.elem)
    }
}

impl <T> PartialOrd for MinNode<T> 
where T: Eq + PartialEq + Ord
{
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Median<T> {
    size: (usize, usize),
    left: BinaryHeap<MaxNode<T>>,
    right: BinaryHeap<MinNode<T>>,
    delayed: HashMap<T, usize>
}

impl <T> Median <T> 
where T: Ord + ToPrimitive + Eq + Hash + Debug
{
    fn new() -> Self {
        Self {
            size: (0,0),
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
            delayed: HashMap::new(),
        }
    }

    fn median(&self) -> f32 {
        let (l, r) = self.size;
        let l_node = self.left.peek();
        let r_node = self.right.peek();
        if l > r {
            return l_node.unwrap().elem.to_f32().unwrap();
        } else if r > l {
            return r_node.unwrap().elem.to_f32().unwrap();
        }

        match (l_node, r_node) {
            (Some(l), Some(r)) => { 
                (l.elem.to_f32().unwrap() + r.elem.to_f32().unwrap()) / 2f32
            },
            (_, _) => { 0f32 },

        }
    }

    fn push(&mut self, elem:T) {
        let lval = self.left.peek();
        if let Some(left) = lval {
            if elem <= left.elem {
                self.left.push(MaxNode{elem});
                self.size.0 += 1;
            } else {
                self.right.push(MinNode{elem});
                self.size.1 += 1;
            }
        } else {
            self.left.push(MaxNode{elem});
            self.size.0 += 1;
        }
        self.rebalance();
    }
    fn rebalance(&mut self) {
        let (l,r) = self.size;
        if r + 1 < l  {
            self.right.push(
            MinNode{elem:self.left.pop().unwrap().elem}
            );
            self.size.0-=1;
            self.size.1+=1;
        } else if l + 1 < r {
            self.left.push(
            MaxNode{elem:self.right.pop().unwrap().elem}
            );
            self.size.0+=1;
            self.size.1-=1;
        }
    }

    fn remove(&mut self, elem:T) {
        if let Some(left) = self.left.peek() {
            if elem <= left.elem {
                self.size.0 -= 1;
            } else {
                self.size.1 -= 1;
            }
        }
        
        *self.delayed.entry(elem).or_insert(0)+=1;
        let mut progress = true;
        while progress {
            progress = false;
            if let Some(left) = self.left.peek() {
                if let Some(entry) = self.delayed.get_mut(&left.elem) {
                    if *entry > 0 {
                        self.left.pop();
                        *entry -= 1;
                        progress = true;
                    }
                }
            }
            if let Some(right) = self.right.peek() {
                if let Some(entry) = self.delayed.get_mut(&right.elem) {
                    if *entry > 0 {
                        self.right.pop();
                        *entry -= 1;
                        progress = true;
                    }
                }
            }
            self.rebalance();
        }
    }
}


fn window_median(k:usize, data:&[usize]) -> Vec<f32> {
    let n = data.len();
    if k > n {
        return vec![];
    }
    let mut med= Median::new();
    let mut medians = Vec::with_capacity(n-k+1);
    for i in 0..k {
        med.push(data[i]);
    }
    medians.push(med.median());
    for i in k..n {
        med.remove(data[i-k]);
        med.push(data[i]);
        medians.push(med.median());
    }
    medians
}


fn window_mean(k:usize, data:&[usize]) -> Vec<f32> {
    let n = data.len();
    if k > n {
        return vec![];
    }
    let mut sum:usize = data[0..k].iter().sum();
    let mut avgs = Vec::with_capacity(n-k+1);
    avgs.push((sum as f32) / (n as f32));
    for idx in k..n {
        sum+= data[idx] - data[idx-k];
        avgs.push((sum as f32) / (n as f32));
    }
    avgs
}

fn main() {
    let mut m = Median::new();
    m.push(1);
    m.push(2);
    m.push(4);
    assert_eq!(m.median(), 2.0);
    m.remove(2);
    assert_eq!(m.median(), 2.5);

    let data = vec![1, 3, 2, 4, 5];
    let k = 3;
    let result = window_mean(k, &data);
    println!("{:?}", result);

    let data = vec![1, 3, 2, 4, 5];
    let k = 3;
    assert_eq!(vec![2f32, 3f32, 4f32], window_median(k, &data));
}

