mod ring;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use ring:: RingBuffer;

#[derive(Eq, PartialEq, Debug)]
struct MinNode<T> {
    elem:T,
}

impl <T> Ord for MinNode<T> 
where T: Ord + PartialOrd,
{
    fn cmp(&self, other:&Self) -> Ordering {
        other.elem.cmp(&self.elem)
    }
}

impl <T> PartialOrd for MinNode<T>
where T: Ord + PartialOrd,
{
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn merge_heap(i:usize, mut j:usize, end:usize, x:&mut [i32]) {
    let mut overflow: BinaryHeap<MinNode<i32>> = BinaryHeap::new();
    let l_end = j;
    for idx in i..=end {
        let l = overflow.peek().map(|n|n.elem).unwrap_or(x[idx]);
        if j > end {
            x[idx] = l;
            continue;
        }
        if l < x[j] {
            if l != x[idx] {
                if i < l_end { overflow.push(MinNode { elem: x[idx] }); }
                x[idx] = overflow.pop().unwrap().elem;
            }
        } else {
            if idx < l_end {
                overflow.push(MinNode { elem: x[idx] });
            }
            x[idx] = x[j];
            j+=1;
        }
    }
}

// if i am in [0..left]
// then throw idx into temp if right < left || temp
// if temp < right => throw idx into temp and idx = temp.pop_front()
// for [right..]
// idx (left) if no longer active, only consider temp
// if j > end, then unpack temp
// if temp == None break

fn merge_ring(i:usize, mut j:usize, end:usize, x:&mut [i32]) {
    let mut overflow  = RingBuffer::new(j - i);
    let boundary = j;
    for idx in i..=end {
        if idx < boundary {
            if let Some(l) = overflow.head() {
                if *l <= x[j] {
                    overflow.push_back(x[idx]);
                    x[idx] = overflow.pop_front().unwrap();
                } else {
                    overflow.push_back(x[idx]);
                    x[idx] = x[j];
                    j+=1;
                }
            } else {
                if x[j] < x[idx] {
                    overflow.push_back(x[idx]);
                    x[idx] = x[j];
                    j+=1;
                }
            }
        } else {
            if let Some(l) = overflow.head() {
                if end < j {
                    x[idx] = *l
                } else if *l < x[j] {
                    x[idx] = *l;
                } else {
                    x[idx] = x[j];
                    j+=1;
                }
                continue;
            };
            break;
        }
    }
}

// fn merge_ring(i:usize, mut j:usize, end:usize, x:&mut [i32]) {
//     let mut overflow  = RingBuffer::new(j - i);
//     let boundary = j;
//     for idx in i..=end {
//         let l = if idx < boundary {
//              overflow.head().unwrap_or(&x[idx])
//         } else {
//              overflow.head().unwrap_or(&i32::MAX)
//         };
//         if j > end {
//             x[idx] = overflow.pop_front().unwrap();
//         } else if *l < x[j] {
//             if *l != x[idx] {
//                 if idx < boundary { 
//                     overflow.push_back(x[idx]);
//                 }
//                 x[idx] = overflow.pop_front().unwrap();
//             }
//         } else {
//             if idx < boundary {
//                 overflow.push_back(x[idx])
//             }
//             x[idx] = x[j];
//             j+=1;
//         }
//     }
// }

fn main() {
    // let mut x = [2,4,1,3];
    // merge_heap(0, 2, 3, &mut x);
    // println!("X {x:?}");
    // let mut x = [1,2,3,4,5,6,7,8];
    // merge_heap(0, 4, 7, &mut x);
    // println!("X {x:?}");
    let mut x = [2,4,1,3];
    merge_ring(0, 2, 3, &mut x);
    println!("X {x:?}");
    let mut x = [1,2,3,4,5,6,7,8];
    merge_ring(0, 4, 7, &mut x);
    println!("X {x:?}");
}
