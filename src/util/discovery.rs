pub mod discovery {
    use robotics_lib::world::World;
    use spyglass::spyglass::SpyglassResult;
    use crate::{ENERGY_BUDGET, MinerRobot, THRESHOLD};

    impl MinerRobot {
        /// Calls the discover_world method if the world hasn't been scanned yet
        ///
        /// # Arguments
        ///
        /// * `world` - the known world
        /// * `distance` - the distance from the robot, indicating the area to discover
        pub fn scan_world(&mut self, world: &mut World, distance: usize) {
            if !self.world_scanned {
                self.discover_world(world, distance);
                self.world_scanned = true;
            }
        }
        /// Discovers the world around the robot
        ///
        /// # Arguments
        ///
        /// * `world` - the known world
        /// * `distance` - the distance from the robot, indicating the area to discover
        fn discover_world(&mut self, world: &mut World, distance: usize) {
            // generating a new spyglass by setting an energy budget and a threshold
            let mut spyglass = self.create_spyglass(world, distance, ENERGY_BUDGET, THRESHOLD);

            // discovering tiles around the robot
            let result = spyglass.new_discover(self, world);

            // managing result
            match result {
                SpyglassResult::Complete(_) => {
                    println!("Scan Complete!");
                },
                SpyglassResult::Failed(_) => {
                    println!("Scan Failed!");
                },
                SpyglassResult::Paused => {
                    println!("Scan Paused!");
                },
                SpyglassResult::Stopped(_) => {
                    println!("Scan Stopped!");
                }
            }
        }
    }
}