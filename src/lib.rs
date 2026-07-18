pub struct Canvas<T> {
    pub width: usize,
    pub height: usize,
    cells: Vec<T>,
}

#[derive(Debug)]
pub enum CanvasError {
    IndexOutOfBounds,
}

impl<T> Canvas<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self
    where
        T: Clone,
    {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![fill; size],
        }
    }

    pub fn within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if !self.within_bounds(x, y) {
            return None;
        }

        let index = x + y * self.width;
        self.cells.get(index)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), CanvasError> {
        if !self.within_bounds(x, y) {
            return Err(CanvasError::IndexOutOfBounds);
        }
        let index = x + y * self.width;
        self.cells[index] = value; // I think this is acceptable because we've guaranteed a valid index.
        Ok(())
    }
}

impl Canvas<u32> {
    pub fn render_into(&self, frame: &mut [u32], scale: usize) {
        for py in 0..self.height * scale {
            for px in 0..self.width * scale {
                let cell_x = px / scale;
                let cell_y = py / scale;
                frame[px + py * (self.width * scale)] = self.cells[cell_x + cell_y * self.width]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_within_bounds_succeeds() {
        let mut canvas = Canvas::<u32>::new(4, 4, 0);
        assert!(canvas.set(1, 1, 0xFF0000).is_ok());
    }

    #[test]
    fn set_out_of_bounds_returns_error() {
        let mut canvas = Canvas::<u32>::new(4, 4, 0);
        assert!(matches!(
            canvas.set(9, 9, 0xFF0000),
            Err(CanvasError::IndexOutOfBounds)
        ));
    }

    #[test]
    fn get_within_bounds_returns_value() {
        let mut canvas = Canvas::<u32>::new(4, 4, 0);
        canvas.set(1, 1, 0xFF0000).unwrap();
        assert_eq!(canvas.get(1, 1), Some(&0xFF0000));
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let canvas = Canvas::<u32>::new(4, 4, 0);
        assert_eq!(canvas.get(9, 9), None);
    }
}
