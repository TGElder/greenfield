use std::fmt::Display;

use num::Float;

pub fn assert_almost_eq<T>(a: T, b: T)
where
    T: Display + Float,
{
    let maybe_zero = a.sub(b);

    if (maybe_zero - T::zero()).abs() > T::epsilon().sqrt() {
        panic!("{} is not almost equal to {}", a, b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn almost_equal() {
        assert_almost_eq(0.5, 1.0 / 2.0);
    }

    #[test]
    #[should_panic(expected = "0.1 is not almost equal to 0.2")]
    fn not_almost_equal() {
        assert_almost_eq(0.1, 0.2);
    }
}
