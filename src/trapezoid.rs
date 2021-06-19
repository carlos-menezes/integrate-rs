use std::io;
use crate::limit::IntgrLimit;

/**
 * 
 * READ: https://en.wikipedia.org/wiki/Trapezoidal_rule
 * Example usage:
 * 
 * let res = trapezoid(|x: f32| (x.powi(4) / 4.0) + x.powi(2) + x.sin(), limit::IntgrLimit { a: 0.0, b: std::f32::consts::PI }, 6);
 * >> 28.44046783447265625000000000000000
 * 
 */

pub fn trapezoid<F>(f: F, l: IntgrLimit, n: i32) -> f32 where
F: Fn(f32) -> f32 {
    let h: f32 = (l.b - l.a) / n as f32;

    let mut s: f32 = 0.0;
    for i in 1..n {
        let x = l.a+(h * i as f32);
        s += f(x);
    }

    return (h / 2.0)*(f(l.a) + (2.0*s) + f(l.b));
}