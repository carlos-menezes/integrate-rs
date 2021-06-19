pub mod limit;
pub mod trapezoid;
pub mod simpson;

#[cfg(test)]
mod tests {
    use super::*;

    use limit::IntgrLimit;
    use trapezoid::trapezoid;
    use simpson::one_third;
    
    #[test]
    fn test_trapezoid() {
        let res = trapezoid(|x: f32| (x.powi(4) / 4.0) + x.powi(2) + x.sin(), IntgrLimit { a: 0.0, b: std::f32::consts::PI }, 6);
        assert_eq!(res, 28.44046783447265625000000000000000);
    }
    
    #[test]
    fn test_simpson_one_third() {
        let res = one_third(|x: f32| (x.powi(4) / 4.0) + x.powi(2) + x.sin(), IntgrLimit { a: 0.0, b: std::f32::consts::PI }, 6);
        assert_eq!(res, 27.64514541625976562500000000000000);
    }
}
