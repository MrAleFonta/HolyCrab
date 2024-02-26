pub mod world {
    // MinerRobot
    use crate::{MinerRobot, World};

    // robotics lib
    use robotics_lib::interface::robot_map;
    use robotics_lib::world::tile::{Content, Tile, TileType};
    impl MinerRobot {
        /// Returns the robot's known map
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        ///
        /// # Returns
        ///
        /// The known world as a Vec<Vec<Tile>>
        pub fn get_map(&self, world: &World) -> Vec<Vec<Tile>> {
            let robot_map = robot_map(world).expect("Error while retrieving the map");
            let world_dim = robot_map.len();
            let default_tile = Tile {
                tile_type: TileType::DeepWater,
                content: Content::None,
                elevation: 0
            };
            let mut map = vec![vec![default_tile; world_dim];world_dim];
            for (i, row) in robot_map.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    match tile {
                        Some(t) => {
                            map[i][j] = t.clone();
                        }
                        None => {
                            map[i][j] = Tile {
                                tile_type: TileType::DeepWater,
                                content: Content::None,
                                elevation: 0
                            };
                        }
                    };
                }
            }
            map
        }
        /// Returns the robot's known map with Option
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        ///
        /// # Returns
        ///
        /// The known world as a Vec<Vec<Option<Tile>>>
        pub fn get_map_option(&self, world: &World) -> Vec<Vec<Option<Tile>>> {
            let robot_map = robot_map(world).expect("Error while retrieving the map");
            let world_dim = robot_map.len();
            let default_tile = Tile {
                tile_type: TileType::DeepWater,
                content: Content::None,
                elevation: 0
            };
            let mut map: Vec<Vec<Option<Tile>>> = vec![vec![Some(default_tile); world_dim];world_dim];
            for (i, row) in robot_map.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    match tile {
                        Some(t) => {
                            map[i][j] = Some(t.clone());
                        }
                        None => {
                            map[i][j] = Some(Tile {
                                tile_type: TileType::DeepWater,
                                content: Content::None,
                                elevation: 0
                            });
                        }
                    };
                }
            }
            map
        }
        /// Returns all the coordinates of tiles that contain the given content
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `content` - the content to search for in the tiles
        ///
        /// # Returns
        ///
        /// A vector of tuples containing the coordinates of tiles with the given content
        pub fn get_tiles_by_content(&self, world: &World, content: Content) -> Vec<(usize, usize)>{
            let mut result = Vec::new();
            for (i, row) in robot_map(world).unwrap().iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    match tile {
                        Some(t) => {
                            if t.content == content {
                                result.push((i,j));
                            }
                        }
                        None => {
                        }
                    };
                }
            }
            result
        }
    }
}