pub mod island {
    // MinerRobot
    use crate::{MinerRobot};

    // robotics lib
    use robotics_lib::world::tile::Tile;

    impl MinerRobot {
        /// Verifies if a position is valid or not
        ///
        /// # Arguments
        ///
        /// * `map` - the known world
        /// * `row` - the row of the coordinate that we are analyzing
        /// * `col` - the column of the coordinate that we are analyzing
        /// * `visited` - a matrix that keeps track of the visited Coordinates
        ///
        /// # Returns
        ///
        /// A bool corresponding to whether moving to that tile is possible or not
        fn is_valid_move(&self, map: &Vec<Vec<Tile>>, row: i32, col: i32, visited: &Vec<Vec<bool>>) -> bool {
            self.is_in_bounds(map,row,col) && self.is_walkable(&map[row as usize][col as usize].tile_type) && !visited[row as usize][col as usize]
        }

        /// Implementation of the Depth-First Search algorithm
        ///
        /// # Arguments
        ///
        /// * `map` - the known world
        /// * `row` - the row of the coordinate that we are analyzing
        /// * `col` - the column of the coordinate that we are analyzing
        /// * `visited` - a matrix that keeps track of the visited Coordinates
        /// * `island_cells` - the vector containing the cells that are part of an island
        fn dfs(&self, map: &Vec<Vec<Tile>>, row: i32, col: i32, visited: &mut Vec<Vec<bool>>, island_cells: &mut Vec<(i32, i32)>) {
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            visited[row as usize][col as usize] = true;
            island_cells.push((row, col));

            for (offset_row, offset_col) in directions {
                let new_row = row + offset_row;
                let new_col = col + offset_col;
                if self.is_valid_move(map, new_row, new_col, visited) {
                    self.dfs(map, new_row, new_col, visited, island_cells);
                }
            }
        }

        /// Implementation of the Depth-First Search algorithm
        ///
        /// # Arguments
        ///
        /// * `map` - the known world
        ///
        /// # Returns
        ///
        /// A vector of islands
        pub fn get_islands(&self, map: &Vec<Vec<Tile>>) -> Vec<Vec<(i32, i32)>> {
            let rows = map.len() as i32;
            let cols = map[0].len() as i32;
            let mut visited = vec![vec![false; cols as usize]; rows as usize];
            let mut islands_cells = Vec::new();

            for i in 0..rows {
                for j in 0..cols {
                    if self.is_walkable(&map[i as usize][j as usize].tile_type) && !visited[i as usize][j as usize] {
                        let mut island_cells = Vec::new();
                        self.dfs(map, i, j, &mut visited, &mut island_cells);
                        islands_cells.push(island_cells);
                    }
                }
            }

            islands_cells
        }
        /// Finds the closest island to the robot's location
        ///
        /// # Arguments
        ///
        /// * `islands` - A vector containing all the islands represented as vectors of (i32,i32)
        ///
        /// # Returns
        ///
        /// An Option of Vec of tuples. The vector represents the closest island to the robot
        pub fn get_closest_island_to_robot(&mut self, islands:  &mut Vec<Vec<(i32, i32)>>) -> Option<Vec<(i32, i32)>> {
            let (robot_row, robot_col) = self.get_coordinates();

            let robot_island = self.get_robot_island(&islands);

            islands.retain(|island| island != &robot_island.clone().unwrap());

            if let Some(_robot_island) = robot_island {
                // finding the island with the coordinate that is closer to the robot
                let closest_island = islands.iter().min_by_key(|island| {
                    island.iter().map(|(row, col)| (row - robot_row as i32).abs() + (col - robot_col as i32).abs()).min().unwrap()
                });

                closest_island.cloned()
            } else {
                None
            }
        }
        /// Finds the closest distance between the robot's island and the target island
        ///
        /// # Arguments
        /// * `map` - the known world
        /// * `robot_island` - the island where the robot stands
        /// * `target_island` - the closest island to the robot
        ///
        /// # Returns
        ///
        /// An option of coordinates indicating the closest walkable tiles that would connect the two islands if there was a bridge
        pub fn get_closest_points(&self, map: &Vec<Vec<Tile>>, robot_island: Vec<(i32, i32)>, target_island: Vec<(i32, i32)>) -> Option<((i32, i32), (i32, i32))> {

            let mut closest_coords = None;
            let mut min_distance = i32::MAX;

            for (target_row,target_col) in target_island {
                for (row,col) in &robot_island {
                    // if the coordinates exist and the Tile is walkable then we check the distance between the target and all the robot's island coordinates
                    if self.is_in_bounds(&map,*row,*col) && self.is_walkable(&map[*row as usize][*col as usize].tile_type) {
                        let distance = (target_row - row).abs() + (target_col - col).abs();
                        if distance < min_distance {
                            min_distance = distance;
                            closest_coords = Some(((target_row,target_col), (*row,*col)));
                        }
                    }
                }
            }

            closest_coords
        }

        /// Returns the island where the robot is located
        ///
        /// # Arguments
        ///
        /// * `islands` - the discovered islands
        ///
        /// # Returns
        ///
        /// An option of Vec of coordinates, indicating the robot's island
        pub fn get_robot_island(&mut self, islands:  &Vec<Vec<(i32, i32)>>) -> Option<Vec<(i32, i32)>>{
            let (robot_row, robot_col) = self.get_coordinates();

            islands.iter()
                    .find(|island| island.contains(&(robot_row as i32, robot_col as i32)))
                    .cloned()
        }
    }
}