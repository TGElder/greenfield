use std::error::Error;

use image::DynamicImage;

pub fn difference(a: &DynamicImage, b: &DynamicImage) -> Result<usize, Box<dyn Error>> {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    if a_bytes.len() != b_bytes.len() {
        return Err(format!(
            "Image data is not same size, {} != {}",
            a_bytes.len(),
            b_bytes.len()
        )
        .into());
    }
    let difference = a_bytes
        .iter()
        .zip(b_bytes)
        .map(|(a, b)| a.abs_diff(*b))
        .map(|i| i as usize)
        .sum::<usize>();
    Ok(difference)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same() {
        // given
        let a = image::open("test_resources/image/similarity-4x4-a.png").unwrap();

        // when
        let result = difference(&a, &a).unwrap();

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn smallest_difference() {
        // given
        let a = image::open("test_resources/image/similarity-4x4-a.png").unwrap();
        let b = image::open("test_resources/image/similarity-4x4-b.png").unwrap();

        // when
        let result = difference(&a, &b).unwrap();

        // then
        assert_eq!(result, 1);
    }

    #[test]
    fn largest_difference() {
        // given
        let black = image::open("test_resources/image/similarity-4x4-black.png").unwrap();
        let white = image::open("test_resources/image/similarity-4x4-white.png").unwrap();

        // when
        let result = difference(&black, &white).unwrap();

        // then
        assert_eq!(result, 12240);
    }

    #[test]
    fn different_size() {
        // given
        let black = image::open("test_resources/image/similarity-4x4-a.png").unwrap();
        let white = image::open("test_resources/image/similarity-3x3-a.png").unwrap();

        // when
        let result = difference(&black, &white);

        // then
        assert!(result.is_err());
    }
}
