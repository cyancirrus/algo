//🔹 1. Numerical Stability & Floating Point
    // IEEE-754, error propagation, Kahan summation
    // Matrix condition numbers, stability of ops like QR/SVD
    // When to use fixed-point or arbitrary precision
    // Why it matters: You’re building tools that manipulate math. Understanding rounding error and representational quirks separates black-box ML from real tool-building.
// 🔹 4. Compilers / DSLs / Codegen
// 🔹 5. Control Theory & Signal Processing
use std::collections::BTreeMap;

struct Range {
    start:u32,
    end:u32,
}
fn merge_intervals(ranges:&mut [Range]) -> &[Range] {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_by_key(|r| r.start);
    let mut res_idx = 0;
    for i in 1..ranges.len() {
        if ranges[i].start <= ranges[res_idx].end {
            ranges[res_idx].end = ranges[res_idx].end.max(ranges[i].end);
        } else {
            res_idx += 1;
            ranges[res_idx].start = ranges[i].start;
            ranges[res_idx].end = ranges[i].end;
        }
    }
    &ranges[0..=res_idx]
}


fn main() {
    // 11 12 21
}
