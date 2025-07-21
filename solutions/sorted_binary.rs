use std::mem;
// ideas for median
// 1. could create a lower::maxheap, higher:: minheap
// 2. might be able to do a binary search with smarts

// lets think with a binary search
// if  x[-1] < y[] then we can pull back like the
// y[ total_len / 2 + x.len() ]
// so boundary cases are pretty straightforward
//
// lets think when they are overlapping
// lets say i find a median in x
// and lets say i find a median in y
//
// i mean also i could just have an i/j index and loop until i + j == idx median
// this one sounds pretty easy as well
// lets do this one quickly and then come back to binary search
//
//okay so how would we do this with binary search
// find median x, median y
// then i would want to look at the lengths
// and then perhaps do somethign like
// if median x > median y
// x -> [0, 1/2], y -> [1/2, 1]
// like do this kinda thing? or what the heck would this find us?
//
// i think that would work?

fn median_sorted_linear(x:&[u8], y:&[u8]) -> u8 {
    let mut i = 0;
    let mut j = 0;
    let mut median = 0; 
    let xlen = x.len();
    let ylen = y.len();
    let total_len = xlen + ylen;
    let low_med_idx = (total_len - 1)>> 1;
    let high_med_idx = (total_len)>> 1;
    loop {
        let idx = i + j;
        if idx == low_med_idx - 1 {
            median += if x[i] <= y[j] {
                i+=1;
                x[i]
            } else {
                j+=1;
                y[j]
            };
            if low_med_idx == high_med_idx {
                return median;
            }
            continue;
        } else if idx == high_med_idx {
            median += if x[i] <= y[j] {
                x[i]
            } else {
                y[j]
            };
            return median / 2;
        }
        if i < xlen && j < ylen {
            if x[i] <= y[j] {
                i+=1;
            } else {
                j += 1;
            };
        } else if j < ylen {
            j = low_med_idx - idx - 1;
        } else if i < xlen {
            i = high_med_idx - idx - 1;
        }
    }
}

// fn median_sorted_binary(x:&mut [u8], y:&mut [u8]) -> u8 {
//     let mut median = 0; 
//     let mut xlen = x.len();
//     let mut ylen = y.len();
//     if xlen > ylen {
//         mem::swap(&mut x, &mut y);
//         mem::swap(&mut xlen, &mut ylen);
//     }
//     let mut idx_med = (xlen + ylen - 1) >> 1;
//     let mut i = (xlen - 1) >> 1;
//     let mut j = (ylen - 1) >> 1;
//     // but we should shrink by the relative size or do we need that? i think unfortunately we do
//     // lets think to keep sizes equal if xlen < ylen (swapped em so they're x always <= y
//     // lets say it's in the upper half of x and in the lower half of y
//     // to ensure we have the same count we should increase size of step in x and decrease step size
//     // in y
//     // also the equality
//     // i + j == median should hold so like say 2/3rds ie xlen is 20 and ylen is 30
//     //
//     // if x[i] > y[j]
//     // i = 10, -> i -= 2/3rd j += 1/3rd
//     // i' -= i * xlen / ylen; j' = idx_med - i'
//     //
//     // if x[i] <= y[j]
//     // i = 10 -> i += 2/3rd j -= 1/3rd
//     // i' += i * xlen / ylen; j' = idx_med - i;

//     loop {
//         if x[i] < y[j] {
//             i += i * xlen / ylen;
//             j = idx_med - i;
//         } else if y[j] < x[i] {
//             i -= i * xlen / ylen;
//             j = idx_med - j;
//         }
//     }
//     median;
// }


fn median_sorted_binary(x:&mut [u8], y:&mut [u8]) -> f32 {
    let xlen = x.len();
    let ylen = y.len();
    let total_len = xlen + ylen;
    let med = (total_len + 1) >> 1;
    let mut low = 0;
    let mut high = xlen;
    let mut i=0;
    let mut j=0;
    let mut median = 0f32;
    while low <= high {
        i = (low + high) >> 1;
        j =  med.wrapping_sub(i);
        // this is actually underflow for y makes i much less
        if j > total_len {
            high = i - 1;
            continue;
        }
        // ensure the deltas don't out of bounds the indexing
        if i < xlen && j > 0 && x[i] < y[j-1] {
            low = i + 1;
        } else if i > 0 && j < ylen && y[j] < x[i-1] {
            high = i - 1;
        } else {
            if i == 0 {
                median = y[j - 1] as f32;
            } else if j == 0 {
                median = x[i - 1] as f32;
            } else {
                median = x[i-1].max(y[j-1]) as f32;
            }
            break
        }
    }
    if total_len & 1 == 1 {
        median
    } else if i == xlen {
        (median + y[j] as f32) / 2f32
    } else if j == ylen {
        (median + x[i] as f32) / 2f32
    } else {
        (median + x[i].min(y[j]) as f32) / 2f32
    }
}


fn main() {
}
