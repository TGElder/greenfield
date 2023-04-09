use std::fmt::Debug;

use num::Float;

#[derive(Debug, PartialEq)]
pub enum Solution<T> {
    RealRoots { min: T, max: T },
    EqualRoots(T),
    ImaginaryRoots,
    NotQuadratic,
}

pub fn solve<T>(a: T, b: T, c: T) -> Solution<T>
where
    T: Float,
    f32: Into<T>,
{
    if a == T::zero() {
        return Solution::NotQuadratic;
    }

    let radicand = b.powi(2) - a.mul(c).mul(4.0.into());

    if radicand > T::zero() {
        let sqrt = radicand.sqrt();
        let denominator = a.mul(2.0.into());
        let a = b.neg().sub(sqrt).div(denominator);
        let b = b.neg().add(sqrt).div(denominator);
        if a < b {
            Solution::RealRoots { min: a, max: b }
        } else {
            Solution::RealRoots { min: b, max: a }
        }
    } else if radicand == T::zero() {
        Solution::EqualRoots(b.neg().div(a.mul(2.0.into())))
    } else {
        Solution::ImaginaryRoots
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use crate::almost::assert_almost_eq;

    use super::*;

    fn test_real_roots<T>(a: T, b: T, c: T)
    where
        T: Debug + Display + From<f32> + Float,
    {
        // when
        let result = solve(a, b, c);

        // then
        let Solution::RealRoots{min, max} = result else {panic!("Expected RealRoots but got {:?}", result)};

        assert!(min <= max);
        test_solution(min, a, b, c);
        test_solution(max, a, b, c);
    }

    fn test_equal_roots<T>(a: T, b: T, c: T)
    where
        T: Debug + Display + From<f32> + Float,
    {
        // when
        let result = solve(a, b, c);

        // then
        let Solution::EqualRoots(root) = result else {panic!("Expected EqualRoots but got {:?}", result)};

        test_solution(root, a, b, c);
    }

    fn test_imaginary_roots<T>(a: T, b: T, c: T)
    where
        T: Debug + Display + From<f32> + Float,
    {
        // when
        let result = solve(a, b, c);

        // then
        assert_eq!(result, Solution::ImaginaryRoots);
    }

    fn test_not_quadratic<T>(a: T, b: T, c: T)
    where
        T: Debug + Display + From<f32> + Float,
    {
        // when
        let result = solve(a, b, c);

        // then
        assert_eq!(result, Solution::NotQuadratic);
    }

    fn test_solution<T>(x: T, a: T, b: T, c: T)
    where
        T: Debug + Display + Float,
    {
        assert_almost_eq(x.powi(2).mul(a).add(x.mul(b)).add(c).abs(), T::zero());
    }

    #[test]
    fn test_1() {
        test_real_roots(-1.0, 1.0, 1.0);
    }

    #[test]
    fn test_2() {
        test_not_quadratic(0.0, 1.0, 1.0);
    }

    #[test]
    fn test_3() {
        test_imaginary_roots(1.0, 1.0, 1.0);
    }

    #[test]
    fn test_4() {
        test_imaginary_roots(2.0, 1.0, 1.0);
    }

    #[test]
    fn test_5() {
        test_imaginary_roots(3.0, 1.0, 1.0);
    }

    #[test]
    fn test_6() {
        test_imaginary_roots(1.0, -1.0, 1.0);
    }

    #[test]
    fn test_7() {
        test_imaginary_roots(1.0, 0.0, 1.0);
    }

    #[test]
    fn test_8() {
        test_equal_roots(1.0, 2.0, 1.0);
    }

    #[test]
    fn test_9() {
        test_real_roots(1.0, 3.0, 1.0);
    }

    #[test]
    fn test_10() {
        test_real_roots(1.0, 1.0, -1.0);
    }

    #[test]
    fn test_11() {
        test_real_roots(1.0, 1.0, 0.0);
    }

    #[test]
    fn test_12() {
        test_imaginary_roots(1.0, 1.0, 2.0);
    }

    #[test]
    fn test_13() {
        test_imaginary_roots(1.0, 1.0, 3.0);
    }

    #[test]
    fn test_f32() {
        test_real_roots(-1.0f32, 1.0f32, 1.0f32);
    }

    #[test]
    fn test_f64() {
        test_real_roots(-1.0f64, 1.0f64, 1.0f64);
    }
}
