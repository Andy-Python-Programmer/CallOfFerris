//! # Call of Ferris
//!
//! Call of Ferris is a thrilling action game where your favorite Ferris the crab and the rust mascot got guns and has taken up the duty to find evildoer languages while managing to keep itself alive.
//! Take part in this awesome adventure and help Ferris be the best ever!
//!
//! For a fuller outline, see the project's [README.md](https://github.com/Andy-Python-Programmer/CallOfFerris)

use std::{fs, rc::Rc, sync::Mutex};

use ggez::{
    conf::WindowMode,
    event::KeyCode,
    event::KeyMods,
    graphics::{set_screen_coordinates, Rect},
    Context, ContextBuilder, GameResult,
};
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use utils::{AssetManager, FerrisResult};

mod screens;
mod utils;

pub use screens::*;

/// Initial window width.
const INIT_WIDTH: f32 = 1000.0;
/// Initial window height.
const INIT_HEIGHT: f32 = 600.0;

/// Minimum width.
const MIN_WIDTH: f32 = 1000.0;
/// Minimum height.
const MIN_HEIGHT: f32 = 600.0;

fn init_assets(ctx: &mut Context) -> FerrisResult<AssetManager> {
    let mut asset_manager = AssetManager::new();

    let images_dir = fs::read_dir("./resources/images/")?;
    let fonts_dir = fs::read_dir("./resources/fonts/")?;
    let audio_dir = fs::read_dir("./resources/audio/")?;
    let maps_dir = fs::read_dir("./resources/maps/")?;

    for image in images_dir {
        asset_manager.load_image(ctx, image?.file_name().to_string_lossy());
    }

    for font in fonts_dir {
        asset_manager.load_font(ctx, font?.file_name().to_string_lossy());
    }

    for audio in audio_dir {
        asset_manager.load_sound(ctx, audio?.file_name().to_string_lossy());
    }

    for map in maps_dir {
        asset_manager.load_file(ctx, "maps", map?.file_name().to_string_lossy());
    }

    Ok(asset_manager)
}

fn main() -> FerrisResult<()> {
    // The resources directory contains all of the assets.
    // Including sprites and audio files.
    let resource_dir = std::path::PathBuf::from("./resources");

    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Call of Ferris", "Borrow Checker")
        .add_resource_path(resource_dir)
        .window_mode(
            WindowMode::default()
                .dimensions(INIT_WIDTH, INIT_HEIGHT)
                .resizable(true)
                .min_dimensions(MIN_WIDTH, MIN_HEIGHT),
        )
        .window_setup(
            WindowSetup::default()
                .title("Call of Ferris")
                .icon("/images/ferris_pacman_1.png"),
        )
        .build()?;

    let asset_manager = init_assets(&mut ctx)?;

    // Create an instance of your event handler.
    let mut game = Game::new(&mut ctx, asset_manager);

    // Run!
    let exit = event::run(&mut ctx, &mut event_loop, &mut game);

    if exit.is_err() {
        let error_message = format!(
            "Call of Ferris encountered an unexpected internal error: {:?}",
            exit,
        );

        Err(error_message.into())
    } else {
        Ok(())
    }
}

/// A enum specifying the current screen to show.
pub enum Screen {
    /// The menu screen.
    Menu,
    /// The game screen.
    Play,
    /// The death screen.
    Dead,
}

/// The current game state.
pub struct Game {
    /// The current screen,
    screen: Screen,
    /// Reference of the menu screen.
    menu_screen: menu::Menu,
    /// Mutable reference of the game screen.
    game_screen: Mutex<game::Game>,
    /// Reference of the death screen.
    death_screen: dead::Death,
    /// The asset manager.
    asset_manager: Rc<AssetManager>,
}

impl Game {
    pub fn new(ctx: &mut Context, asset_manager: AssetManager) -> Self {
        let asset_manager = Rc::new(asset_manager);

        // Woah. We are cloning the asset manager. Yes thats why its wrapped in Rc<>
        // Anything wrapped in a Rc<> and performs a clone it only clones its pointer so its fine to use clone here!
        Self {
            screen: Screen::Menu,

            menu_screen: menu::Menu::create(ctx, asset_manager.clone()),
            game_screen: game::Game::create(ctx, asset_manager.clone()),
            death_screen: dead::Death::spawn(ctx, asset_manager.clone()),

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
                            self.game_screen = game::Game::create(ctx, self.asset_manager.clone());
                        }

                        _ => (),
                    }

                    self.screen = s;
                }
            }
            Screen::Dead => {}
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).unwrap();
    }
}
