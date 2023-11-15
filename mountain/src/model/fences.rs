use commons::geometry::XY;
use thiserror::Error;

#[derive(Default)]
pub struct Fences {
    data: [bool; 4],
}

#[derive(Debug, Error)]
#[error("Invalid delta: {delta}")]
pub struct InvalidDelta<'a> {
    delta: &'a XY<i32>,
}

impl Fences {
    pub fn _is_fence<'a>(&self, delta: &'a XY<i32>) -> Result<bool, InvalidDelta<'a>> {
        Ok(self.data[Self::index(delta)?])
    }

    pub fn toggle_fence<'a>(&mut self, delta: &'a XY<i32>) -> Result<bool, InvalidDelta<'a>> {
        let is_fence = &mut self.data[Self::index(delta)?];
        *is_fence = !*is_fence;
        Ok(*is_fence)
    }

    fn index(delta: &XY<i32>) -> Result<usize, InvalidDelta> {
        match delta {
            XY { x: 1, y: 0 } => Ok(0),
            XY { x: 0, y: 1 } => Ok(1),
            XY { x: -1, y: 0 } => Ok(2),
            XY { x: 0, y: -1 } => Ok(3),
            _ => Err(InvalidDelta { delta }),
        }
    }
}
