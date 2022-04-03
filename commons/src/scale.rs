use num::Float;
use std::borrow::Borrow;

pub struct Scale<T>
where
    T: Float,
{
    from: (T, T),
    to: (T, T),
}

impl<T> Scale<T>
where
    T: Float,
{
    pub fn new(from: (T, T), to: (T, T)) -> Scale<T> {
        Scale { from, to }
    }

    pub fn scale<R>(&self, value: R) -> T
    where
        R: Borrow<T>,
    {
        let value = value.borrow();

        let from_length = self.from.1 - self.from.0;
        let to_length = self.to.1 - self.to.0;

        let p = (*value - self.from.0) / from_length;

        p * to_length + self.to.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_inside_range() {
        let scale = Scale::<f64>::new((1.0, 11.0), (0.0, 4.0));

        assert_eq!(scale.scale(8.5), 3.0);
    }

    #[test]
    fn test_scale_above_input_range() {
        let scale = Scale::<f64>::new((1.0, 11.0), (0.0, 4.0));

        assert_eq!(scale.scale(13.5), 5.0);
    }

    #[test]
    fn test_scale_below_input_range() {
        let scale = Scale::<f64>::new((1.0, 11.0), (0.0, 4.0));

        assert_eq!(scale.scale(-1.5), -1.0);
    }
}
