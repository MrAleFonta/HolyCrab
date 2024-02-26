use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::usize;

// modules for MinerRobot
mod util;

// robotics lib
use robotics_lib::event::events::Event;
use robotics_lib::interface::destroy;
use robotics_lib::interface::Direction;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::energy::Energy;
use robotics_lib::world::World;

// tools
use bessie::bessie::{road_paving_machine, RpmError, State};
use bob_lib::tracker::GoalTracker;
use colored::Colorize;
use OwnerSheeps_Sound_Tool::functions::weather_sounds::weather_sound;
use pmp_collect_all::CollectAll;
use robotics_lib::utils::LibError;
use rust_and_furious_dynamo::dynamo::Dynamo;
use sense_and_find_by_rustafariani::Lssf;
use spyglass::spyglass::Spyglass;


// const used to set the goal quantity
const GOAL_QUANTITY: u32 = 5;

// const that are used for the spyglass
const ENERGY_BUDGET: usize = 300;
const THRESHOLD: f64 = 0.5;

// scan distance and increase (indicates how much the distance will increase if changed)
pub const SCAN_DISTANCE: usize = 10;
const SCAN_INCREASE: usize = 10;

// energy threshold, the robot's energy cannot get lower than the threshold
const MIN_ENERGY: usize = 100;

#[derive(Debug)]
pub enum RobotState {
    CollectingRocks,
    PavingBridge
}

// struct of the robot
pub struct MinerRobot {
    pub robot: Robot,
    pub name: String,
    pub goal_tracker: GoalTracker,
    pub rocks_collected: usize,
    pub scan_distance: usize,
    pub lssf: Lssf,
    pub world_scanned: bool,
    pub state: RobotState,
    pub game_over: Arc<Mutex<bool>>,
    pub coordinates:(usize, usize)
}

impl MinerRobot {
    /// Constructors
    ///

    /// Creates a new instance of MinerRobot
    ///
    /// # Returns
    ///
    /// A new instance of Self
    pub fn new() -> Self {
        Self {
            robot: Robot::new(),
            name: String::from("The default miner"),
            goal_tracker: GoalTracker::new(),
            rocks_collected: 0,
            scan_distance: SCAN_DISTANCE,
            lssf: Lssf::new(),
            world_scanned: false,
            state: RobotState::CollectingRocks,
            game_over: Arc::new(Mutex::from(false)),
            coordinates: (0,0)
        }
    }
    /// Creates a new instance of MinerRobot given its name
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the robot
    ///
    /// # Returns
    ///
    /// A new instance of Self
    pub fn new_name(name: String) -> Self {
        Self {
            robot: Robot::new(),
            name,
            goal_tracker: GoalTracker::new(),
            rocks_collected: 0,
            scan_distance: SCAN_DISTANCE,
            lssf: Lssf::new(),
            world_scanned: false,
            state: RobotState::CollectingRocks,
            game_over: Arc::new(Mutex::from(false)),
            coordinates:(0,0)
        }
    }

    /// Utility methods
    ///

    /// Generates and returns the instance of a new Spyglass
    ///
    /// # Arguments
    ///
    ///  * `world` - the world
    ///  * `distance` - the distance from the center within which the discovery is to take place
    ///  * `energy_budget` - the maximum amount of energy the tool can use
    ///  * `threshold` - value that determines whether to discover a certain area
    ///
    /// # Returns
    ///
    /// An instance of the newly created Spyglass
    fn create_spyglass(&mut self, world: &World, distance: usize, energy_budget: usize, threshold: f64) -> Spyglass {
        let mut spyglass = Spyglass::new_default(
            self.robot.coordinate.get_row(),
            self.robot.coordinate.get_col(),
            distance,
            self.get_map(world).len()
        );
        spyglass.set_energy_budget(Some(energy_budget));
        spyglass.set_view_threshold(threshold);

        spyglass
    }
    /// Updates both the Lssf map and cost
    ///
    /// # Arguments
    ///
    /// * `map` - the map of the discovered world
    /// * `row` - the row coordinate from which we want the cost to be updated
    /// * `col` - the column coordinate from which we want the cost to be updated
    fn update_lssf_map_and_cost(&mut self, world: &mut World, map: &Vec<Vec<Option<Tile>>>, row: usize, col: usize) {
        self.lssf.update_map(&map);
        match self.lssf.update_cost(row,col) {
            Ok(()) => {
                //println!("Lssf cost updated successfully")
            },
            Err(e) => {
                self.catch_lib_error(world,e);
            }
        }
    }
    /// Calls the Bessie tool and starts collecting rocks
    ///
    /// # Arguments
    ///
    ///  * `world` - the world
    ///  * `direction` - the direction in which the robot starts paving the road
    fn collect_rocks_inline(&mut self, world: &mut World, direction: Direction) {
        match road_paving_machine(self,world,direction,State::GetStones) {
            Ok(()) => {
                //println!("The Process ended correctly and we made a Road!")
            }
            Err(e) => {
                self.catch_rpm_error(world,e);
            }
        }
    }
    /// Collects all the content around the robot
    ///
    /// # Arguments
    ///
    /// * `world` - the world
    /// * `range` - the range around the robot
    fn collect_all(&mut self, world: &mut World, range: usize) {
        CollectAll::collect_all(self,world,range);
    }
    /// Recharges the energy if the energy level goes below the minimum threshold
    ///
    /// # Arguments
    ///
    /// * `world` - the world
    fn manage_energy(&mut self, world: &mut World) {
        if self.robot.energy.get_energy_level() < MIN_ENERGY {
            self.recharge_energy(world);
        }
    }
    /// Recharges the robot's energy by calling the Dynamo tool
    ///
    /// # Arguments
    ///
    /// * `world` - the world
    fn recharge_energy(&mut self, world: &mut World) {
        let _= destroy(self, world, Direction::Down);
        *self.get_energy_mut() = Dynamo::update_energy();
    }
    /// Updates self's rock count
    fn update_rock_count(&mut self) {
        let contents = self.robot.backpack.get_contents();
        if let Some(&rocks_amount) = contents.get(&Content::Rock(0)) {
            self.rocks_collected = rocks_amount;
        }
    }
    /// Returns the robot's coordinates
    ///
    /// # Returns
    ///
    /// The robot's coordinates as a tuple
    pub fn get_coordinates(&self) -> (usize,usize) {
        (self.robot.coordinate.get_row(),self.robot.coordinate.get_col())
    }
    /// Catches the LibError
    ///
    /// # Arguments
    ///
    /// * `world` - the world
    /// * `error` - the LibError
    pub fn catch_lib_error(&mut self, world: &mut World, error: LibError) {
        match error {
            LibError::NotEnoughEnergy => {
                println!("Not enough energy, the robot will get its energy refilled");
                self.recharge_energy(world);
            },
            LibError::OutOfBounds => println!("Out of bounds"),
            LibError::NoContent => println!("No content"),
            LibError::NotEnoughSpace(remainder) => println!("Not enough space: {}", remainder),
            LibError::CannotDestroy => println!("Cannot destroy"),
            LibError::NotCraftable => println!("Can't craft this item"),
            LibError::NoMoreDiscovery => println!("Not enough discoverable tiles"),
            _ => println!("Generic error: {:?}", error)
        }
    }
    /// Catches the RpmError for the road paving machine
    ///
    /// # Arguments
    ///
    /// * `world` - the world
    /// * `error` - the RpmError
    pub fn catch_rpm_error(&mut self, world: &mut World, error: RpmError) {
        match error {
            RpmError::NotEnoughEnergy => {
                println!("Not enough energy, the robot will get its energy refilled");
                self.recharge_energy(world);
            },
            RpmError::CannotPlaceHere => println!("Cannot place content on the current tile"),
            RpmError::OutOfBounds => println!("Out of bounds"),
            RpmError::NotEnoughMaterial => println!("Not enough material"),
            RpmError::NoRockHere => println!("No rock here"),
            RpmError::MustDestroyContentFirst => println!("Must destroy content first"),
            RpmError::UndefinedError => println!("Undefined error")
        }
    }
    /// Sets the game_over value to true, ending the game
    pub fn game_is_over(&mut self) {
        let mut game_over_lock = self.game_over.lock().unwrap();

        // Modify the boolean value
        *game_over_lock = true;
    }
}

impl Runnable for MinerRobot {
    fn process_tick(&mut self, world: &mut World) {
        weather_sound(world);

        // scanning the area around the robot once
        self.scan_world(world,self.scan_distance);

        // managing the creation/deletion of goals
        self.handle_goals();

        // self.print_discovered_tiles_content(&world);
        //self.print_discovered_tiles_tile_type(&world);

        // moving and collecting rocks
        self.move_and_collect_content(world, Content::Rock(1));

        // building a bridge if possible
        self.pave_bridge(world);

        // if the robot's energy drops below a certain threshold it recharges
        self.manage_energy(world);

    }
    #[allow(dead_code)]
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(_) => {}
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(tile, coordinates) => {
                let mut coordinate = self.coordinates;
                coordinate = coordinates;
            }
            Event::TileContentUpdated(_, coordinate) => {}
            Event::AddedToBackpack(_, _) => {}
            Event::RemovedFromBackpack(_, _) => {}
        }
    }
    #[allow(dead_code)]
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    #[allow(dead_code)]
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    #[allow(dead_code)]
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    #[allow(dead_code)]
    fn get_coordinate_mut(&mut self) -> &mut Coordinate{
        &mut self.robot.coordinate
    }
    #[allow(dead_code)]
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    #[allow(dead_code)]
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

/// Implementing the Debug trait for the MinerRobot
impl Debug for MinerRobot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               format!("{}'s STATS\n\
               - Coordinates: {:?} \n\
               - Energy: {:?} \n\
               - Goal_tracker: \n \
                    \t - Goals: {:?} \n \
                    \t - Completed: {} \n\
               - Rocks collected: {:?} \n\
               - State: {:?} \n",
                       self.name, self.get_coordinates(), self.robot.energy.get_energy_level(),
                       self.goal_tracker.get_goals(), self.goal_tracker.get_completed_number(),
                       self.rocks_collected, self.state
               ).green()
        )
    }
}
