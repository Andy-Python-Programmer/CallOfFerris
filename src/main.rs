//! # Call of Ferris
//!
//! Call of Ferris is a thrilling action game where your favorite Ferris the crab and the rust mascot got guns and has taken up the duty to find evildoer languages while managing to keep itself alive.
//! Take part in this awesome adventure and help Ferris be the best ever!
//!
//! For a fuller outline, see the project's [README.md](https://github.com/Andy-Python-Programmer/CallOfFerris)

use std::{fs, rc::Rc, sync::Mutex};

use ggez::{conf::WindowMode, event::KeyCode, event::KeyMods, Context, ContextBuilder, GameResult};
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use utils::AssetManager;

mod screens;
mod utils;

pub use screens::*;

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 600.0;

fn load_assets(ctx: &mut Context) -> AssetManager {
    let mut asset_manager = AssetManager::new();

    let images_dir = fs::read_dir("./resources/images/").unwrap();
    let fonts_dir = fs::read_dir("./resources/fonts/").unwrap();
    let audio_dir = fs::read_dir("./resources/audio/").unwrap();
    let maps_dir = fs::read_dir("./resources/maps/").unwrap();

    for image in images_dir {
        asset_manager.load_image(ctx, image.unwrap().file_name().to_str().unwrap());
    }

    for font in fonts_dir {
        asset_manager.load_font(ctx, font.unwrap().file_name().to_str().unwrap());
    }

    for audio in audio_dir {
        asset_manager.load_sound(ctx, audio.unwrap().file_name().to_str().unwrap());
    }

    for map in maps_dir {
        asset_manager.load_file(ctx, "maps", map.unwrap().file_name().to_str().unwrap());
    }

    asset_manager
}

fn main() -> GameResult<()> {
    // The resources directory contains all of the assets.
    // Including sprites and audio files.
    let resource_dir = std::path::PathBuf::from("./resources");

    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Call of Ferris", "Borrow Checker")
        .add_resource_path(resource_dir)
        .window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT))
        .window_setup(
            WindowSetup::default()
                .title("Call of Ferris")
                .icon("/images/ferris_pacman_1.png"),
        )
        .build()?;

    let asset_manager = load_assets(&mut ctx);

    // Create an instance of your event handler.
    let mut game = Game::new(&mut ctx, asset_manager);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => Ok(println!("Exited cleanly.")),
        Err(e) => Ok(println!("Error occured: {}", e)),
    }
}

pub enum Screen {
    Menu,
    Play,
    Dead,
}

pub struct Game {
    screen: Screen,
    menu_screen: menu::menu::Menu,
    game_screen: Mutex<game::game::Game>,
    death_screen: dead::dead::Death,

    asset_manager: Rc<AssetManager>,
}

impl Game {
    pub fn new(ctx: &mut Context, asset_manager: AssetManager) -> Self {
        let asset_manager = Rc::new(asset_manager);

        // Woah. We are cloning the asset manager. Yes thats why its wrapped in Rc<>
        // Anything wrapped in a Rc<> and performs a clone it only clones its pointer so its fine to use clone here!
        Self {
            screen: Screen::Menu,

            menu_screen: menu::menu::Menu::create(ctx, asset_manager.clone()),
            game_screen: game::game::Game::create(ctx, asset_manager.clone()),
            death_screen: dead::dead::Death::spawn(ctx, asset_manager.clone()),

            asset_manager,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {
            match self.screen {
                Screen::Menu => self.menu_screen.update(ctx)?,
                Screen::Play => {
                    let change = self.game_screen.lock().unwrap().update(ctx)?;

                    if let Some(s) = change {
                        self.screen = s;
                    }
                }
                Screen::Dead => self.death_screen.update(ctx)?,
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.screen {
            Screen::Menu => self.menu_screen.draw(ctx),
            Screen::Play => {
                let change = self.game_screen.lock().unwrap().draw(ctx)?;

                if let Some(s) = change {
                    self.screen = s;
                }

                Ok(())
            }
            Screen::Dead => self.death_screen.draw(ctx),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match self.screen {
            Screen::Play => self.game_screen.lock().unwrap().key_up_event(keycode),
            _ => (),
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match self.screen {
            Screen::Menu => {
                let change = self.menu_screen.key_press(keycode);

                if let Some(s) = change {
                    self.screen = s;
                }
            }
            Screen::Play => {
                let change = self.game_screen.lock().unwrap().key_press(keycode);

                if let Some(s) = change {
                    match s {
                        Screen::Menu => {
                            self.game_screen =
                                game::game::Game::create(ctx, self.asset_manager.clone());
                        }

                        _ => (),
                    }

                    self.screen = s;
                }
            }
            Screen::Dead => {}
        }
    }
}
