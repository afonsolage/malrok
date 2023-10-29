#[derive(Debug, Clone)]
pub struct Heightmap {
    width: u16,
    depth: u16,
    buffer: Vec<i16>,
}

impl Heightmap {
    pub fn new(width: u16, depth: u16) -> Self {
        Heightmap {
            width,
            depth,
            buffer: vec![0; width as usize * depth as usize],
        }
    }

    pub fn index(&self, x: u16, z: u16) -> usize {
        x as usize * self.depth as usize + z as usize
    }

    pub fn position(&self, index: usize) -> (u16, u16) {
        (
            (index / self.depth as usize) as u16,
            (index % self.depth as usize) as u16,
        )
    }

    pub fn get(&self, x: u16, z: u16) -> i16 {
        self.buffer[self.index(x, z)]
    }

    pub fn set(&mut self, x: u16, z: u16, value: i16) {
        let index = self.index(x, z);
        self.buffer[index] = value;
    }
}

impl std::ops::Index<usize> for Heightmap {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl std::ops::IndexMut<usize> for Heightmap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_height_map() {
        let width = 3;
        let depth = 4;
        let heightmap = Heightmap::new(width, depth);

        assert_eq!(heightmap.width, width);
        assert_eq!(heightmap.depth, depth);

        for value in &heightmap.buffer {
            assert_eq!(*value, 0);
        }
    }

    #[test]
    fn test_index_and_position() {
        let heightmap = Heightmap::new(3, 3);

        let index = heightmap.index(1, 2);
        let position = heightmap.position(index);

        assert_eq!(index, 5);
        assert_eq!(position, (1, 2));
    }

    #[test]
    fn test_get_and_set() {
        let mut heightmap = Heightmap::new(3, 3);

        heightmap.set(1, 2, 42);
        let value = heightmap.get(1, 2);

        assert_eq!(value, 42);
    }

    #[test]
    fn test_index_operator() {
        let mut heightmap = Heightmap::new(3, 3);

        heightmap[5] = 42;
        let value = heightmap[5];

        assert_eq!(value, 42);
    }
}
