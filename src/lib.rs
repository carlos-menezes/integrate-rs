pub mod limit;
pub mod trapezoid;

#[cfg(test)]
mod tests {
    use super::*;

    use limit::IntgrLimit;
    use trapezoid::{ trapezoid };
    
    #[test]
    fn test_fn_1() {
        let res = trapezoid(|x: f32| (x.powi(4) / 4.0) + x.powi(2) + x.sin(), IntgrLimit { a: 0.0, b: std::f32::consts::PI }, 6);
        assert_eq!(res, 28.44046783447265625000000000000000);
    }
}
