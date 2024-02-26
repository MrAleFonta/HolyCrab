#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use worldgen_unwrap::public::WorldgeneratorUnwrap;
    use robotics_lib::world::world_generator::Generator;
    #[test]
    fn test_world_creation() {
        let gui_start = false;
        let path = PathBuf::new().join("world.bin");
        let mut world_generator = WorldgeneratorUnwrap::init(gui_start, Some(path));
        let world = world_generator.gen();
        println!("")


    }
}

fn main() {

}