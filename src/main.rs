use std::str::from_utf8;
use std::mem;

fn evaluate_point(poly:&[f64], x:f64) -> f64 {
    let mut result = 0f64;
    for (p, a) in poly.iter().enumerate() {
        result += a * x.powf(p as f64)
    }
    result
}

fn find_derivative(poly:&[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(poly.len()-1);
    for (p, a) in poly[1..].iter().enumerate() {
        result.push(poly[p] * a )
    }
    result
}

fn quick_scan(poly:&[f64], y:f64) -> f64 {
    let mut estimate = 0f64;
    let dt = find_derivative(&poly);
    let epsilon = 1e-2;
    loop {
        let eval = evaluate_point(poly, estimate);
        let err = eval - y;
        if err.abs() > epsilon {
            println!("err {err:}");
            estimate -= err / evaluate_point(&dt, estimate);
            continue;
        } 
        return estimate
    }
}

fn numeric_diff<F:Fn(f64) -> f64>(f:F, x:f64) -> f64 {
    let epsilon = 1e-2;
    (f(x+epsilon) - f(x)) / epsilon
}

fn secant_diff<F:Fn(f64) -> f64>(f:F, x:f64) -> f64 {
    let epsilon = 1e-2;
    (f(x+epsilon) - f(x-epsilon)) / (epsilon + epsilon)
}


fn main() {
    // x^2 + x + 2 = 0;
    let poly = [2f64, 1f64, 1f64];
    // println!("Result for 14 {:?}", quick_scan(&poly, 14f64));
    // should might want to add in updates for like if notice loop, b/c it will bounce around if
    // not in range
    println!("Result for 14 {:?}", quick_scan(&poly, +1212352.21));
    // // assert_eq!(3, len_post_trans(&"za", 25));
    // assert_eq!(3, len_post_trans(&"z", 26));
    // assert_eq!(2, len_post_trans(&"z", 1));
    // for t in 0..100 {
    //     let input = "zaz";
    //     let a = len_post_math(input, t);
    //     let mut s = input.bytes().collect();
    //     for _ in 0..t {
    //         worker(&mut s);
    //     }
    //     let b = s.len();
    //     assert_eq!(a, b, "Failed at t = {}: math = {}, sim = {}", t, a, b);
    // }
}
