pub mod path_find {
    // MinerRobot
    use crate::{MinerRobot, RobotState, SCAN_INCREASE};

    // robotics lib
    use robotics_lib::interface::{destroy, go, Direction};
    use robotics_lib::world::{tile::Content, World};

    // tools
    use sense_and_find_by_rustafariani::Action;
    use bob_lib::tracker::*;
    use colored::Colorize;
    use OwnerSheeps_Sound_Tool::functions::destroying_sound::play_sound_mining_rock;

    const RANGE: usize = 2;
    const DIRECTION: Direction = Direction::Up;

    impl MinerRobot {
        /// Moves the robot to a target tile and collects the specified Content present in that tile
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `content` - the content that we want to collect
        ///
        /// # Notes
        ///
        /// If the coordinates are the same after calling the move it means that the content vector is empty.
        /// The way that this issue is handled is by:
        /// - increasing the distance
        /// - setting the scanned value to false in order to call the discover once again
        /// - getting all the content around the robot
        /// - trying to collect rocks
        /// we increase the distance and reset the scanned value to false in order to call the discover once again
        pub fn move_and_collect_content(&mut self, world: &mut World, content: Content) {

            // getting the vector that contains the cost to reach tiles from the robot's coordinates
            let vec = self.get_cost_vector_to_content(world, content);

            // moving the robot on the target tile and collecting the content
            let (row,col) = self.get_coordinates();
            self.state = RobotState::CollectingRocks;
            self.move_to_tile_destroy_content(world, vec);
            let (new_row,new_col) = self.get_coordinates();

            if (row,col) == (new_row,new_col) {
                // increasing the scan of the spyglass and resetting the world_scanned value in order to scan it again
                println!("{}", "Increased scan".green());
                self.scan_distance += SCAN_INCREASE;
                self.world_scanned = false;

                // setting conditions to use the collect_rocks method and to end the game
                self.collect_all(world,RANGE);
                if self.scan_distance > (SCAN_INCREASE * 4) {
                    self.game_is_over();
                    println!("{:?}", self);
                } else if self.scan_distance > (SCAN_INCREASE * 3) {
                    self.collect_rocks_inline(world,DIRECTION);
                }
            }
        }
        /// Generates and returns the vector that associates coordinates containing Content, with the cost to reach them
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `content` - the content that we want to look for
        ///
        /// # Returns
        ///
        /// A vector of tuples:
        /// - the first element represents the cost to reach the tile
        /// - the second element represents the coordinates of the tile
        pub fn get_cost_vector_to_content(&mut self, world: &mut World, content: Content) -> Vec<(usize,(usize,usize))>{
            let mut cost_vector: Vec<(usize,(usize,usize))> = Vec::new();

            let map = self.get_map_option(world);
            let (x,y) = self.get_coordinates();

            // updating both map and costs
            self.update_lssf_map_and_cost(world, &map, x, y);

            // getting the vector that contains all the coordinates of tiles that contain a specific content
            let content_vec = self.get_tiles_by_content(world,content);

            // adding both cost and coordinates to the cost vector by iterating over the content vector
            for (row,col) in content_vec {
                match self.lssf.get_cost(row,col){
                    Some(cost) => {
                        if (x,y) != (row,col) {
                            cost_vector.push((cost,(row,col)));
                        }
                    },
                    None => {}
                };
            }

            // we order the cost vector so that the first element is the one with the lesser cost
            cost_vector.sort();
            cost_vector
        }

        /// Moves towards a target tile and destroys its content
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `vec` - the vector of tuples (cost(row,col))
        ///
        /// # Notes
        ///
        /// The robot moves until it reaches the tile near the target, and then it destroys the target's content
        pub fn move_to_tile_destroy_content(&mut self, world: &mut World, vec: Vec<(usize, (usize, usize))>) {
            let (_cost,(x,y));
            // if the vector is not empty then we take the first element which is the one that costs less to go to
            if vec.len() > 0 {
                (_cost,(x,y)) = vec[0];
            } else { // otherwise we return
                println!("The content vector is empty");
                return;
            }

            let action_vec = match self.lssf.get_action_vec(x,y){
                Ok(vec) => vec,
                Err(e) => {
                    self.catch_lib_error(world,e);
                    return ();
                }
            };

            for (i,action) in action_vec.iter().enumerate() {
                let direction = self.action_to_direction(action);
                // calling the destroy if the robot is facing the tile containing Content
                if i == action_vec.len() - 1 {
                    match destroy(self, world, direction.clone()) {
                        Ok(quantity) => {
                            play_sound_mining_rock();
                            // updating the rock count and the goal tracker
                            self.update_rock_count();
                            self.goal_tracker.update_manual(GoalType::GetItems,Some(Content::Rock(1)),quantity);
                        }
                        Err(e) => {
                            self.catch_lib_error(world,e);
                        }
                    } ;
                }
                // moving the robot to Direction and returning an error message in case of failure
                let msg = format!("Failed to move {:?}", direction);
                self.manage_energy(world);
                go(self,world,direction.clone()).expect(msg.as_str());
            }
        }
        /// Converts an action into a direction
        ///
        /// # Arguments
        ///
        /// `action`: the action to convert
        ///
        /// # Returns
        ///
        /// The corresponding direction
        pub fn action_to_direction(&self, action: &Action) -> Direction {
            match action {
                Action::North => Direction::Up,
                Action::East => Direction::Right,
                Action::South => Direction::Down,
                Action::West => Direction::Left,
                Action::Teleport(_x,_y) => Direction::Up,
            }
        }
    }
}