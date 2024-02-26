/* 
pub mod debug {
    // MinerRobot
    use crate::MinerRobot;

    // robotics lib
    use robotics_lib::world::World;
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use robotics_lib::interface::robot_map;

    // other
    use colored::Colorize;
    impl MinerRobot {
        /// Prints all the discovered tiles content
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        pub fn print_discovered_tiles_content(&self, world: &World) {
            print!("- ");
            for (i, _row) in robot_map(world).unwrap().iter().enumerate() {
                print!("{} ", i % 10);
            }
            println!();
            for (i, row) in robot_map(world).unwrap().iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    if j == 0 {
                        print!("{} ", i % 10);
                    }
                    if i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col() {
                        print!("! ");
                    } else {
                        match tile {
                            None => {
                                print!("- ")
                            }
                            Some(t) => {
                                Self::print_content(t.clone().content)
                            }
                        };
                    }
                }
                println!();
            }
            println!();
        }
        /// Decides whether to print the tiles tile_type in unicode or not based on the world dimension
        pub fn print_discovered_tiles_tile_type(&self, world: &World) {
            let map = robot_map(world).unwrap();
            if map.len() < 30 {
                self.print_discovered_tiles_tile_type_unicode(&map);
            } else {
                self.print_discovered_tiles_tile_type_default(&map);
            }
        }
        /// Prints all the discovered tiles tile_type
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        fn print_discovered_tiles_tile_type_default(&self, map: &Vec<Vec<Option<Tile>>>) {
            print!("- ");
            for (i, _row) in map.iter().enumerate() {
                print!("{} ", i % 10);
            }
            println!();
            for (i, row) in map.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    if j == 0 {
                        print!("{} ", i % 10);
                    }
                    if i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col() {
                        print!("! ");
                    } else {
                        match tile {
                            None => {
                                print!("- ")
                            }
                            Some(t) => {
                                Self::print_tile_type(t.clone().tile_type)
                            }
                        };
                    }
                }
                println!();
            }
            println!();
        }
        /// Prints all the discovered tiles tile_type in unicode
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        fn print_discovered_tiles_tile_type_unicode(&self, map: &Vec<Vec<Option<Tile>>>) {
            print!("{:<4} ","- ");
            for (i, _row) in map.iter().enumerate() {
                print!("{:<4} ", i % 10);
            }
            println!();
            for (i, row) in map.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    if j == 0 {
                        print!("{:<4} ", i % 10);
                    }
                    if i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col() {
                        print!("{:<4}","\u{1F916}");
                    } else {
                        match tile {
                            None => {
                                print!("{:<4} ","-")
                            }
                            Some(t) => {
                                Self::print_tile_type_unicode(t.clone().tile_type)
                            }
                        };
                    }
                }
                println!();
            }
            println!();
        }
        /// Prints the respective letter to the content given
        ///
        /// # Arguments
        ///
        /// * `content` - the content that is getting printed
        fn print_content(content: Content) {
            print!("{}", match content {
                Content::Bank(_) => { "A ".yellow() }
                Content::Bin(_) => { "I ".yellow() }
                Content::Building => { "B ".yellow() }
                Content::Bush(_) => { "H ".yellow() }
                Content::Crate(_) => { "C ".yellow() }
                Content::Coin(_) => { "O ".yellow() }
                Content::Fire => { "F ".yellow() }
                Content::Fish(_) => { "P ".yellow() }
                Content::Garbage(_) => { "G ".yellow() }
                Content::JollyBlock(_) => { "J ".yellow() }
                Content::Market(_) => { "M ".yellow() }
                Content::Rock(_) => { "R ".green() }
                Content::Scarecrow => { "S ".yellow() }
                Content::Tree(_) => { "T ".yellow() }
                Content::Water(_) => { "W ".yellow() }
                Content::None => { "+ ".yellow() }
            })
        }
        /// Prints the respective letter to the TileType given
        ///
        /// # Arguments
        ///
        /// * `tile_type` - the TileType that is getting printed
        fn print_tile_type(tile_type: TileType) {
            print!("{}", match tile_type {
                TileType::DeepWater => { "D ".blue() }
                TileType::Grass => { "G ".green() }
                TileType::Hill => { "H ".green() }
                TileType::Lava => { "L ".green() }
                TileType::Mountain => { "M ".green() }
                TileType::Sand => { "S ".green() }
                TileType::ShallowWater => { "o ".blue() }
                TileType::Snow => { "N ".green() }
                TileType::Street => { "R ".red() }
                TileType::Teleport(_) => { "T ".green() }
                TileType::Wall => { "W ".green() }
            })
        }
        /// Prints the respective letter to the TileType given in unicode
        ///
        /// # Arguments
        ///
        /// * `tile_type` - the TileType that is getting printed
        fn print_tile_type_unicode(tile_type: TileType) {
            print!("{:<4}", match tile_type {
                TileType::DeepWater => { "\u{1F30A}"}
                TileType::Grass => { "\u{1F33F}" }
                TileType::Hill => { "\u{26F0}" }
                TileType::Lava => { "\u{1F30B}"}
                TileType::Mountain => { "\u{1F3D4}" }
                TileType::Sand => { "\u{1F3D6}" }
                TileType::ShallowWater => { "o " }
                TileType::Snow => { "\u{2744}"}
                TileType::Street => { "\u{1F309}"}
                TileType::Teleport(_) => { "\u{1F504}" }
                TileType::Wall => { "\u{1F6A1}" }
            })
        }
    }
}
*/