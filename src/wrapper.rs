use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Runnable;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use holy_crab_best_path::MinerRobot;

pub struct BotWrapper{
    pub bot: MinerRobot
}

impl BotWrapper{
    pub(crate) fn new(bot: MinerRobot) ->BotWrapper{
        BotWrapper{
            bot
        }
    }
}

impl Runnable for BotWrapper{
    fn process_tick(&mut self, world: &mut World) {
        self.bot.process_tick(world)
    }

    fn handle_event(&mut self, event: Event) {
        self.bot.handle_event(event)
    }

    fn get_energy(&self) -> &Energy {
        self.bot.get_energy()
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        self.bot.get_energy_mut()
    }

    fn get_coordinate(&self) -> &Coordinate {
        self.get_coordinate()
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        self.get_coordinate_mut()
    }

    fn get_backpack(&self) -> &BackPack {
        self.get_backpack()
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        self.get_backpack_mut()
    }
}