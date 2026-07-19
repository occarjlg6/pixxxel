pub struct Palette {
    colors: [(u32, &'static str); 8],
    primary_index: usize,
    secondary_index: usize,
}

#[derive(Debug)]
pub enum PaletteError {
    IndexOutOfBounds,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors: [
                (0xFFFFFF, "White"),
                (0x000000, "Black"),
                (0xFF0000, "Red"),
                (0x00FF00, "Green"),
                (0x0000FF, "Blue"),
                (0xFFFF00, "Yellow"),
                (0xFF00FF, "Magenta"),
                (0x00FFFF, "Cyan"),
            ],
            primary_index: 0,
            secondary_index: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    pub fn color_at(&self, index: usize) -> Option<u32> {
        if index < self.colors.len() {
            Some(self.colors[index].0)
        } else {
            None
        }
    }

    pub fn color_name_at(&self, index: usize) -> Option<&'static str> {
        if index < self.colors.len() {
            Some(self.colors[index].1)
        } else {
            None
        }
    }

    pub fn primary_index(&self) -> usize {
        self.primary_index
    }

    pub fn set_primary_index(&mut self, index: usize) -> Result<(), PaletteError> {
        if index >= self.colors.len() {
            return Err(PaletteError::IndexOutOfBounds);
        }
        self.primary_index = index;
        Ok(())
    }

    pub fn secondary_index(&self) -> usize {
        self.secondary_index
    }

    pub fn set_secondary_index(&mut self, index: usize) -> Result<(), PaletteError> {
        if index >= self.colors.len() {
            return Err(PaletteError::IndexOutOfBounds);
        }
        self.secondary_index = index;
        Ok(())
    }

    // The setters reject out-of-range indices, so both fields always point at a
    // real entry and these four lookups can never fail.
    pub fn primary_color(&self) -> u32 {
        self.colors[self.primary_index].0
    }

    pub fn primary_color_name(&self) -> &'static str {
        self.colors[self.primary_index].1
    }

    pub fn secondary_color(&self) -> u32 {
        self.colors[self.secondary_index].0
    }

    pub fn secondary_color_name(&self) -> &'static str {
        self.colors[self.secondary_index].1
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}
