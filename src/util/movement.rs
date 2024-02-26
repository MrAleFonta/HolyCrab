pub mod movement {
    // MinerRobot
    use crate::MinerRobot;

    // robotics lib
    use robotics_lib::interface::{Direction, go};
    use robotics_lib::world::tile::{Tile, TileType};
    use robotics_lib::world::World;
    impl MinerRobot {
        /// Moves the robot to the given coordinates
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `map` - the known world
        /// * `coordinates` - the target coordinates
        pub fn move_to_coords(&mut self, world: &mut World, map: &Vec<Vec<Tile>>, coordinates: (i32,i32)) {
            let (row,col) = coordinates;

            // retrieving the action vector, which will indicate the robot the directions to go to
            let action_vec = match self.lssf.get_action_vec(row as usize,col as usize){
                Ok(vec) => vec,
                Err(e) => {
                    self.catch_lib_error(world,e);
                    return ();
                }
            };

            // iterating through all the actions that will lead the robot to the target coordinates
            for action in action_vec {
                let (robot_row, robot_col) = self.get_coordinates();
                let direction = self.action_to_direction(&action);

                // calculating the new row and column
                let (row_offset, col_offset) = self.direction_to_offset(&direction);
                let target_row = robot_row as i32 + row_offset;
                let target_col = robot_col as i32 + col_offset;

                // checking if the coordinates are within bounds and if the tile_type is walkable
                if self.is_in_bounds(&map,target_row,target_col) && self.is_walkable(&map[target_row as usize][target_col as usize].tile_type) {
                    let msg = format!("Failed to move {:?}", direction);
                    self.manage_energy(world);
                    go(self,world,direction.clone()).expect(msg.as_str());
                }
            }
        }
        /// Checks if a tile is walkable or not
        ///
        /// # Arguments
        ///
        /// * `tile_type` - the target TileType
        ///
        /// # Returns
        ///
        /// A bool stating whether the tile is walkable or not
        pub fn is_walkable(&self, tile_type: &TileType) -> bool {
            match tile_type {
                TileType::DeepWater => false,
                TileType::Lava => false,
                TileType::ShallowWater => false,
                _ => true
            }
        }
        /// Converts the direction into an offset
        ///
        /// # Arguments
        ///
        /// * `direction` - the direction
        ///
        /// # Returns
        ///
        /// An i32 tuple representing the offset
        pub fn direction_to_offset(&self, direction: &Direction) -> (i32,i32){
            match direction {
                Direction::Up => (-1,0),
                Direction::Left => (0,-1),
                Direction::Down => (1,0),
                Direction::Right => (0,1),
            }
        }
        /// Checks if the row value and the column value are within the matrix bounds
        ///
        /// # Arguments
        ///
        /// * `map` - the known world
        /// * `row` - the row to be verified
        /// * `col` - the column to be verified
        ///
        /// # Returns
        ///
        /// A bool representing whether the values are in bounds or not
        pub fn is_in_bounds(&self, map: &Vec<Vec<Tile>>, row: i32, col: i32) -> bool {
            let rows = map.len() as i32;
            let cols = map[0].len() as i32;
            row >= 0 && col >= 0 && row < rows && col < cols
        }
    }
}