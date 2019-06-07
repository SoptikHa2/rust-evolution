/// World map
pub struct Map {
    food: Vec<isize>,
    width: usize,
    height: usize,
}

impl Map {
    /// Find nearest food from coordinates and return direction to it.
    ///
    /// If it doesn't exist, return None.
    pub fn find_nearest_food_direction(x: usize, y: usize) -> Option<Direction> {}

    /// Return map description.
    ///
    /// This returns a three-value tuple.
    /// First element is map width, second one
    /// is map height, and third one is `&Vec<isize>`, which
    /// describes where on the map is food.
    pub fn read_map(&self) -> (usize, usize, &Vec<isize>) {
        (self.width, self.height, &self.food)
    }

    /// Try to get food value at coordinates.
    fn get_food_at(&self, x: usize, y: usize) -> Option<isize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        } else {
            return Some(self.food[y * self.width + x]);
        }
    }
}

/// This describes direction to any item
/// which we want to find.
pub enum Direction {
    Here,
    North,
    South,
    West,
    East,
}
