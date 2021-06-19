use crate::limit::IntgrLimit;

pub fn one_third<F>(f: F, l: IntgrLimit, n: i32) -> f32 where
F: Fn(f32) -> f32 {
    let h: f32 = (l.b - l.a) / n as f32;

    let mut s1: f32 = 0.0; // for odd `i`
    for i in (1..n).step_by(2) {
        let x = l.a+(h * i as f32);
        s1 += f(x);
    }

    let mut s2: f32 = 0.0; // for even `i`
    for i in (2..n-1).step_by(2) {
        let x = l.a+(h * i as f32);
        s2 += f(x);
    }

    return (h / 3.0)*(f(l.a) + 4.0*s1 + 2.0*s2 + f(l.b))
}

pub fn three_eights<F>(f: F, l: IntgrLimit, n: i32) -> f32 where
F: Fn(f32) -> f32 {
    let h: f32 = (l.b - l.a) / n as f32;

    let mut s1: f32 = 0.0; // for i != 3, 6, 9, ...
    for i in 1..n {
        if i % 3 != 0 {
            let x = l.a+(h * i as f32);
            s1 += f(x);
        }
    }

    let mut s2: f32 = 0.0; // for i = 3, 6, 9, ...
    for i in (3..n-2).step_by(2) {
        let x = l.a+(h * i as f32);
        s2 += f(x);
    }

    return (3.0 * h / 8.0)*(f(l.a) + 3.0*s1 + 2.0*s2 + f(l.b))
}