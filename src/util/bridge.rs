pub mod debug {
    // MinerRobot
    use crate::{MinerRobot, RobotState};

    // robotics lib
    use robotics_lib::interface::{go, put};
    use robotics_lib::interface::Direction;
    use robotics_lib::world::tile::{Content, TileType};
    use robotics_lib::world::World;
    use robotics_lib::world::tile::Tile;

    // tools
    use OwnerSheeps_Sound_Tool::functions::put_sounds::{play_sound_rock_in_g_h_s_s, play_sound_rock_in_lava, play_sound_rock_in_water};

    impl MinerRobot {
        /// Builds the bridge if the collected rocks are enough and if the target doesn't change with time
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        ///
        /// # Notes
        ///
        /// The robot performs a certain amount of iterations to make sure that the target is correct:
        /// - we move the robot to the starting tile
        /// - once the robot is on the starting tile we calculate the bridge points one more time:
        ///     - if they change it means that the starting tile is somewhere else, and we repeat the process
        ///     - if they stay the same we start building the bridge
        pub fn pave_bridge(&mut self, world: &mut World) {
            let (mut target_island_coords, mut robot_island_coords) = self.calculate_bridge_points(world);

            // we want to make sure that the target is the right one, so we iterate n amount of times
            let mut iterations = 0;
            let max_iterations = 10;

            while iterations < max_iterations {
                let (robot_row,robot_col) = self.get_coordinates();
                // checking the collected rock's amount
                let rocks_to_build_bridge = self.get_paving_cost(&self.get_map(world), robot_island_coords, target_island_coords);
                if self.rocks_collected < rocks_to_build_bridge {
                    break;
                }
                // if the robot is not on the starting tile to build the bridge, we move it there
                if (robot_row as i32, robot_col as i32) != robot_island_coords {
                    self.move_to_coords(world, &self.get_map(world), robot_island_coords);
                }
                let (new_target_island_coords, new_robot_island_coords) = self.calculate_bridge_points(world);
                if new_target_island_coords == target_island_coords {
                    self.start_building_bridge(world, target_island_coords);
                    self.rocks_collected -= rocks_to_build_bridge;
                    break;
                } else {
                    target_island_coords = new_target_island_coords;
                    robot_island_coords = new_robot_island_coords;
                }
                iterations += 1;
            }
            // the game ends if there have been too many iterations without finding the correct tile
            if iterations >= max_iterations {
                println!("The target keeps on changing {}", max_iterations);
                self.game_is_over();
                println!("{:?}", self);
            }
        }
        /// Starts building the bridge and sets the robot's state to PavingBridge
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `(target_island_row,target_island_col)` - the target island's coordinates
        fn start_building_bridge(&mut self, world: &mut World, (target_island_row,target_island_col): (i32, i32)) {
            self.state = RobotState::PavingBridge;
            self.build_along_row_and_col(world, target_island_row, target_island_col);
        }
        /// Calculates the coordinates that will be connected by the bridge
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        ///
        /// # Returns
        ///
        /// A tuple of coordinates indicating the two coordinates that will be at the start and at the end of the bridge
        fn calculate_bridge_points(&mut self, world: &World) -> ((i32,i32),(i32,i32)) {
            let mut islands = self.get_islands(&self.get_map(world));
            let discovered_tiles = self.get_map(world);

            // getting both the robot's island and the target island
            let robot_island = self.get_robot_island(&islands).unwrap_or_else(|| vec![]);
            let target_island = self.get_closest_island_to_robot(&mut islands).unwrap_or_else(|| vec![(0, 0)]);

            self.get_closest_points(&discovered_tiles, robot_island, target_island).unwrap_or_else(|| ((0, 0), (0, 0)))
        }
        /// Calls the method to build a bridge on both rows and columns
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `map` - the known world
        /// * `row` - the target's row
        /// * `col` - the target's column
        ///
        /// # Notes:
        ///
        /// Given the distance between the robot's row coordinate and the target's it calls the build_to_direction() method.
        /// The same goes for the column.
        fn build_along_row_and_col(&mut self, world: &mut World, row: i32, col: i32) {
            let (robot_row,robot_col) = self.get_coordinates();
            let row_distance = robot_row as i32 - row;
            let col_distance = robot_col as i32 - col;

            // building following rows
            if row_distance < 0 {
                self.build_to_direction(world,row_distance,&Direction::Down);
            } else if row_distance > 0 {
                self.build_to_direction(world,row_distance,&Direction::Up);
            }
            // building following columns
            if col_distance < 0 {
                self.build_to_direction(world,col_distance,&Direction::Right);
            } else if col_distance > 0 {
                self.build_to_direction(world,col_distance,&Direction::Left);
            }
            if row_distance == 0 && col_distance == 0 {
                println!("Cannot build since the robot is already on the target tile");
            }
        }
        /// Builds a bridge given a direction and a distance
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `map` - the known world
        /// * `distance` - the amount of blocks that are getting paved
        /// * `direction` - the direction that the robot will pave on
        fn build_to_direction(&mut self, world: &mut World, distance: i32, direction: &Direction) {
            let mut distance_left = distance.abs();

            // iterating through all the tiles that need to connect the robot to the target except for the last one which is the target tile
            while distance_left > 1 {
                let map = self.get_map(world);
                let (mut row,mut col) = self.get_coordinates();

                // calculating the offset in order to find the next tile's coordinates
                let (offset_row,offset_col) = self.direction_to_offset(&direction);
                row = ((row as i32) + offset_row) as usize;
                col = ((col as i32) + offset_col) as usize;

                // calculating the amount of rocks needed to build the bridge
                let quantity = self.get_tile_cost(&map[row][col].tile_type);

                // calling put to pave the bridge if the coordinates are within bounds and the tile is not walkable
                let error = if self.is_in_bounds(&map,row as i32,col as i32) && !self.is_walkable(&map[row][col].tile_type) {
                    put(self, world, Content::Rock(0), quantity, direction.clone())
                } else {
                    Ok(0)
                };
                let _ = match error {
                    Ok(_) => {
                        self.play_sound_paving(&map[row][col].tile_type);
                        self.manage_energy(world);
                        // in case of error a message is returned
                        let msg = format!("Failed to move {:?}", direction);
                        go(self,world,direction.clone()).expect(msg.as_str());
                        Ok(())
                    },
                    Err(e) => {
                        self.catch_lib_error(world, e);
                        Err(())
                    }
                };

                distance_left -= 1;
            }
        }
        /// Calculates the total cost of building a bridge from the robot's coordinates to the given ones
        ///
        /// # Arguments
        ///
        /// * `map` - the known map
        /// * `(robot_row,robot_col)` - the robot's coordinates
        /// * `(island_row, island_col)` - the target's coordinates
        ///
        /// # Returns
        ///
        /// The cost of building a bridge from the robot's coordinates to the target's
        pub fn get_paving_cost(&self, map: &Vec<Vec<Tile>>, (robot_row,robot_col): (i32,i32), (island_row, island_col): (i32, i32)) -> usize {
            // initializing the total cost variable and the tmp_cost (indicates the last evaluated cost)
            let mut cost = 0;
            let mut curr_cost = 0;

            let (mut curr_row, mut curr_col) = (robot_row,robot_col);

            // loop through all the coordinates that separate the current coordinates to the target's
            // since the robot builds first along rows and then along columns, we start calculating the rows first
            while curr_row != island_row {
                let next_row = if (curr_row) < island_row {
                    curr_row + 1
                } else if (curr_row) > island_row {
                    curr_row - 1
                } else {
                    curr_row
                };
                curr_cost = self.get_tile_cost(&map[next_row as usize][curr_col as usize].tile_type);
                cost += curr_cost;
                curr_row = next_row;
            }
            while curr_col != island_col {
                let next_col = if (curr_col) < island_col {
                    curr_col + 1
                } else if (curr_col) > island_col {
                    curr_col - 1
                } else {
                    curr_col
                };
                curr_cost = self.get_tile_cost(&map[curr_row as usize][next_col as usize].tile_type);
                cost += curr_cost;
                curr_col = next_col;
            }
            // subtracting the curr_cost since it is the cost of the last tile which is the target tile
            // and the robot doesn't need to build a bridge there
            cost-curr_cost
        }
        /// Calls the sound tool based on the tile_type
        fn play_sound_paving(&self, tile_type: &TileType) {
            match tile_type {
                TileType::DeepWater => play_sound_rock_in_water(),
                TileType::ShallowWater => play_sound_rock_in_water(),
                TileType::Lava => play_sound_rock_in_lava(),
                _ => play_sound_rock_in_g_h_s_s(),
            }
        }
        /// Returns the cost of paving a certain tile
        ///
        /// # Arguments
        ///
        /// * `tile_type` - the TileType that we are looking at
        ///
        /// # Returns
        ///
        /// The cost of paving a tile with the given TileType
        fn get_tile_cost(&self, tile_type: &TileType) -> usize {
            match tile_type {
                TileType::DeepWater => 3,
                TileType::Lava => 3,
                TileType::ShallowWater => 2,
                TileType::Mountain => 0,
                _ => 1
            }
        }
    }
}