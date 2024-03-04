use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use colored::Colorize;
use ggez::conf;
use ggez::context::Has;
use ggez::input::keyboard::KeyCode;
use ggez::timer::sleep;
use ggez::GameError;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Image};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::backpack;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Runnable, Runner};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use worldgen_unwrap::public::WorldgeneratorUnwrap;
use holy_crab_best_path::MinerRobot;

const SCREEN_SIZE: f32 = 1500.;
const DESIRED_FPS: u32 = 1;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE, SCREEN_SIZE))
        .window_setup(conf::WindowSetup::default().title("Holy Crab!"))
        .build()
        .expect("aieee, could not create ggez context!");

    // Creazione del canale per la comunicazione tra thread
    let (sender, receiver) = mpsc::channel();

    // Creazione della struttura MyGame
    let my_game = MyGame::new(&mut ctx, sender.clone(), receiver)?;

    event::run(ctx, event_loop, my_game)
}

struct MyGame {
    map: Vec<Vec<Tile>>,
    images: HashMap<TileType, Image>,
    image_robot: Image,
    image_rock: Image,
    receiver: mpsc::Receiver<(f32,f32,f32,f32)>, // Canale per ricevere le coordinate del robot
    len_x: f32, 
    len_y: f32,
    offset: (f32,f32),
    key_pressed: bool
}

impl MyGame {
    fn new(
        ctx: &mut Context,
        sender: mpsc::Sender<(f32, f32, f32,f32)>, // Aggiungi sender come parametro
        receiver: mpsc::Receiver<(f32, f32, f32,f32)>, // Aggiungi receiver come parametro
    ) -> GameResult<MyGame> {
        let mut hs = HashMap::new();
        hs.insert(TileType::DeepWater, Image::from_path(ctx,"/tiles/Map_tile_37.png")?);
        hs.insert(TileType::ShallowWater, Image::from_path(ctx,"/tiles/Map_tile_01.png")?);
        hs.insert(TileType::Grass, Image::from_path(ctx,"/tiles/Map_tile_23.png")?);
        hs.insert(TileType::Hill, Image::from_path(ctx,"/tiles/hill1.png")?);
        hs.insert(TileType::Sand, Image::from_path(ctx,"/tiles/sand.png")?);
        hs.insert(TileType::Lava, Image::from_path(ctx,"/tiles/Map_tile_110.png")?);
        hs.insert(TileType::Snow, Image::from_path(ctx,"/tiles/Map_tile_23.png")?);
        hs.insert(TileType::Mountain, Image::from_path(ctx,"/tiles/Map_tile_23.png")?);
        hs.insert(TileType::Street, Image::from_path(ctx,"/tiles/Map_tile_23.png")?);

        let gui_start = false;
        let path = PathBuf::new().join("world/bridge2.bin");
        let mut world_generator = WorldgeneratorUnwrap::init(gui_start, Some(path));
        let world = world_generator.gen();

        let map = world.0;

        // Inizializza len_x e len_y con i valori appropriati
        let len_y = SCREEN_SIZE / map.len() as f32;
        let len_x = SCREEN_SIZE / map[0].len() as f32;

        // Avvio del thread che gestisce la logica del robot
        let my_robot = MinerRobot::new();

        thread::spawn(move || {
            // Accedi al MinerRobot all'interno del Mutex
            let my_robot_box = Box::new(my_robot);

            let run = Runner::new(my_robot_box, &mut world_generator); // Usa borrow_mut per ottenere il riferimento mutabile all'interno del Mutex
            match run {
                Ok(mut running) => {
                    loop {
                        // Invia le coordinate del robot attraverso il canale
                        let coord = running.get_robot().get_coordinate(); // Usa borrow per ottenere il riferimento immutabile all'interno del Mutex
                        let backpack = running.get_robot().get_backpack().get_contents();
                        let number_of_rocks =  *backpack.get(&Content::Rock(0)).unwrap();
                        let level_energy = running.get_robot().get_energy().get_energy_level();
                        if let Err(e) = sender.send((coord.get_row() as f32, coord.get_col() as f32, level_energy as f32,number_of_rocks as f32)) {
                            println!("Error sending robot coordinates: {:?}", e);
                        }
                        let _ = running.game_tick();
                        sleep(Duration::from_millis(1000));
                    }
                }
                Err(e) => {
                    println!("Error in runnable - main");
                    println!("{:?}", e);
                }
            }
        });

        Ok(MyGame {
            map: map,
            images: hs,
            image_robot: Image::from_path(ctx,"/objects/elf.png")?,
            image_rock: Image::from_path(ctx,"/objects/prova_rock.png")?,
            receiver: receiver, // Ricevi il ricevitore del canale come parametro
            len_x: len_x, // Inizializza len_x
            len_y: len_y, // Inizializza len_y
            offset: (0.,0.),
            key_pressed: false
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        if !self.key_pressed {
            // Verifica lo stato dei tasti e aggiorna lo stato di gioco di conseguenza
            if ctx.keyboard.is_key_pressed(KeyCode::W) {
                // Zoom in
                // Riduci la dimensione della cella della mappa
                // Ad esempio, dimezza le dimensioni della cella
                self.len_x *= 1.5;
                self.len_y *= 1.5;
                self.key_pressed = true;
            }

            if ctx.keyboard.is_key_pressed(KeyCode::S) {
                // Zoom out
                // Aumenta la dimensione della cella della mappa
                // Ad esempio, raddoppia le dimensioni della cella
                self.len_x /= 1.5;
                self.len_y /= 1.5;
                self.key_pressed = true;
            }

            if ctx.keyboard.is_key_pressed(KeyCode::Up) {
                // Zoom out
                // Aumenta la dimensione della cella della mappa
                // Ad esempio, raddoppia le dimensioni della cella
                self.offset.1 -= 1.0;
                self.key_pressed = true;
            }

            if ctx.keyboard.is_key_pressed(KeyCode::Down)  {
                // Zoom out
                // Aumenta la dimensione della cella della mappa
                // Ad esempio, raddoppia le dimensioni della cella
                self.offset.1 += 1.0;
                self.key_pressed = true;
            }

            if ctx.keyboard.is_key_pressed(KeyCode::Left) {
                // Zoom out
                // Aumenta la dimensione della cella della mappa
                // Ad esempio, raddoppia le dimensioni della cella
                self.offset.0 -= 1.0;
                self.key_pressed = true;
            }

            if ctx.keyboard.is_key_pressed(KeyCode::Right) {
                // Zoom out
                // Aumenta la dimensione della cella della mappa
                // Ad esempio, raddoppia le dimensioni della cella
                self.offset.0 += 1.0;
                self.key_pressed = true;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            println!("------ Sono dentro la funzione draw! ------");
            println!("{} {} {} {} {}",self.len_x,self.len_y,self.offset.0,self.offset.1,self.key_pressed);
            let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));
            if let Ok(coord) = self.receiver.try_recv() {                
                // Disegna la mappa partendo dall'alto dello spazio vuoto
                let mut index_x = 0.0;
                let mut index_y = 0.;
                for row in &self.map {
                    for tile in row {
                        let draw_param = DrawParam::new()
                            .dest(Vec2::new(index_x - self.offset.0 * self.len_x, index_y - self.offset.1 * self.len_y))
                            .scale(Vec2::new(self.len_x, self.len_y));
        
                        canvas.draw(self.images.get(&tile.tile_type).unwrap(), draw_param);
                        match &tile.content {
                            Content::Rock(_) => {
                                let draw_param = DrawParam::new()
                                    .dest(Vec2::new(index_x - self.offset.0 * self.len_x + self.len_x/3.0, index_y - self.offset.1 * self.len_y  + self.len_y/4.0))
                                    .scale(Vec2::new(self.len_x / 360.0, self.len_y / 340.));
                                canvas.draw(&self.image_rock, draw_param);
                            },
                            _ => {}
                        }
                        
                        index_x += self.len_x
                    }
                    index_x = 0.0;
                    index_y += self.len_y;
                }
                // Disegna il robot con le nuove coordinate
                let robot_dest = Vec2::new(coord.1 * self.len_y - self.offset.0 * self.len_x, coord.0 * self.len_x + self.len_x/4. - self.offset.1 * self.len_y,);
                canvas.draw(&self.image_robot, DrawParam::default().dest(robot_dest).scale(Vec2::new(self.len_x / 500.,self.len_y / 500.)));
                // Tolgo dalla mappa la roccia
                self.map[coord.0 as usize][coord.1 as usize].content = Content::None;
                // Disegna il rettangolo rosso nello spazio vuoto in cima alla schermata
                let text = graphics::Text::new(format!("Energy: "));
                let text_dest = Vec2::new(10.0, 20.0);
                canvas.draw(&text, DrawParam::new().dest(text_dest));
                // Calcola la larghezza del rettangolo rosso in base alla percentuale desiderata (0.0 - 1.0)
                let max_width = 200.0; // Larghezza massima del rettangolo

                // Imposta le coordinate e le dimensioni del rettangolo più grande (con bordi visibili)
                let big_rect_dest = graphics::Rect::new(text_dest.x + 70.0, text_dest.y - 5.0, max_width, 25.0);
                let big_rect_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(2.0), // Imposta lo spessore del bordo a 2.0
                    big_rect_dest,
                    Color::WHITE, // Colore del bordo
                )?;

                // Calcola la larghezza del rettangolo rosso in base alla percentuale e crea il rettangolo rosso
                let red_rect_width = coord.2 / 5.0;
                let red_rect_dest = graphics::Rect::new(text_dest.x + 70.0, text_dest.y - 5.0, red_rect_width, 25.0);
                let red_rect_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    red_rect_dest,
                    Color::RED,
                )?;
                // Disegna prima il rettangolo più grande (con bordo) e poi il rettangolo rosso sopra di esso
                canvas.draw(&big_rect_mesh, DrawParam::default());
                canvas.draw(&red_rect_mesh, DrawParam::default());

                let text = graphics::Text::new(format!("BackPack: "));
                let text_dest = Vec2::new(1000.0, 20.0);
                canvas.draw(&text, DrawParam::new().dest(text_dest));

                for i in 0..coord.3 as usize {
                    let draw_param = DrawParam::new()
                        .dest(Vec2::new(1075.0 + 20.0*(i as f32+1.0 as f32), 15.0))
                        .scale(Vec2::new(self.len_x / 360.0, self.len_y / 340.));
                    canvas.draw(&self.image_rock, draw_param);
                }
            }
            canvas.finish(ctx)?;
            self.key_pressed = false;
        }
        Ok(())
    }
}

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
        match event {
            Event::Ready => todo!(),
            Event::Terminated => todo!(),
            Event::TimeChanged(_) => todo!(),
            Event::DayChanged(_) => todo!(),
            Event::EnergyRecharged(_) => todo!(),
            Event::EnergyConsumed(_) => todo!(),
            Event::Moved(_, _) => todo!(),
            Event::TileContentUpdated(_, _) => todo!(),
            Event::AddedToBackpack(_, _) => todo!(),
            Event::RemovedFromBackpack(_, _) => todo!(),
        }
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