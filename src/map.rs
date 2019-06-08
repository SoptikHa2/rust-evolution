use std::cmp::max;

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
    pub fn find_nearest_food_direction(&self, x: usize, y: usize) -> Option<Direction> {
        // If there is food on current tile, return Direction::Here
        {
            if let Some(food_on_this_tile) = self.get_food_at(x, y) {
                // If there is at least some food on this tile,
                // return Direction::Here.
                if food_on_this_tile > 0 {
                    return Some(Direction::Here);
                }
            } else {
                // If curent coordinates are not valid, return none
                return None;
            }
        }

        let maximum_distance: usize = max(self.width, self.height);
        let food_found: Option<(usize, usize)> = None;
        // food_found is coordinate of nearest available food.
        for distance in 0..maximum_distance {
            // Distance is current scan distance from x, y.
            // This distance will increment until we run out of map
            // or food is found
            for dx in 0..distance + 1 {
                for dy in 0..distance + 1 {
                    // If we already found food, get out of these loops.
                    if food_found.is_some() {
                        break;
                    }

                    // `dx and `dy` represents the change for x and y
                    // coordinates.
                    // For example for distance: 2, the both dx
                    // and dy will be 0, 1, 2. We throw out
                    // every combination of dx, dy that doesn't contain
                    // `distance` value (in this case: 2), and than
                    // use both positive and negative dx, dy.
                    // We need to be careful not to overflow the value.
                    // Afterwards, we check if there is food and if so,
                    // return direction.

                    // We need either dx or dy to be equal to distance.
                    if dx != distance && dy != distance {
                        continue;
                    }

                    // Check if we don't overflow for positive dx, dy
                    if x + dx < self.width && y + dy < self.height {
                        if let Some(food) = self.get_food_at(x + dx, y + dy) {
                            if food > 0 {
                                food_found = Some((x + dx, y + dy));
                                break;
                            }
                        }
                    }

                    // Check if we don't overflow for negative dx, dy
                    if dx < x && dy < y {
                        if let Some(food) = self.get_food_at(x - dx, y - dy) {
                            if food > 0 {
                                food_found = Some((x - dx, y - dy));
                                break;
                            }
                        }
                    }
                }
            }
        }

        if let Some(food_found) = food_found {
            if food_found.0 > x {
                Some(Direction::East)
            } else if food_found.0 < x {
                Some(Direction::West)
            } else if food_found.1 > y {
                Some(Direction::South)
            } else if food_found.1 < y {
                Some(Direction::North)
            } else {
                None
            }
        } else {
            None
        }
    }

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
