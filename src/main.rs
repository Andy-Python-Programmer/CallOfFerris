// https://www.reddit.com/r/rust/comments/k35yy7/call_of_ferris_ownership_war/

use std::sync::Mutex;

use ggez::{conf::WindowMode, event::KeyCode, event::KeyMods, Context, ContextBuilder, GameResult};
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};

mod dead;
mod game;
mod menu;
mod utils;
mod map;

mod components {
    pub mod barrel;
    pub mod bullet;
    pub mod cloud;
    pub mod enemy;
    pub mod player;
    pub mod tile;
}

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 600.0;

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

    // Create an instance of your event handler.
    let mut game = MyGame::new(&mut ctx);

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

pub struct MyGame {
    screen: Screen,
    menu_screen: menu::Menu,
    game_screen: Mutex<game::Game>,
    death_screen: dead::Death,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            screen: Screen::Menu,

            menu_screen: menu::Menu::create(ctx),
            game_screen: game::Game::create(ctx),
            death_screen: dead::Death::spawn(ctx),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.screen {
            Screen::Menu => self.menu_screen.update(ctx),
            Screen::Play => {
                let change = self.game_screen.lock().unwrap().update(ctx)?;

                if let Some(s) = change {
                    self.screen = s;
                }

                Ok(())
            }
            Screen::Dead => self.death_screen.update(ctx),
        }
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
            },
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
        _ctx: &mut Context,
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
                    self.screen = s;
                }
            }
            Screen::Dead => {}
        }
    }
}
