use std::{sync::Mutex, process::exit};

use ggez::{conf::WindowSetup, audio::{SoundSource, Source}, event::{self, EventHandler}};
use ggez::graphics;
use ggez::{conf::WindowMode, event::KeyCode, event::KeyMods, Context, ContextBuilder, GameResult};

mod menu;
mod game;
mod dead;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

fn main() {
    let resource_dir = std::path::PathBuf::from("./resources");

    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Call of Ferris", "Borrow Checker")
        .add_resource_path(resource_dir)
        .window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT))
        .window_setup(WindowSetup::default().title("Call of Ferris"))
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

enum Screen {
    Menu,
    Play,
    Dead
}

pub struct MyGame {
    ferris_borrow_fail: graphics::Image,
    #[allow(dead_code)]
    ferris_planet: graphics::Image,
    screen: Screen,
    game: Option<Mutex<game::Game>>,
    velocity: f64,
    consolas: graphics::Font,
    #[allow(dead_code)]
    ferris_dead: Source
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> Self {
        let ferris_borrow_angry = graphics::Image::new(ctx, "/ferris_borrow_angry.png").unwrap();
        let ferris_planet = graphics::Image::new(ctx, "/ferris_planet.png").unwrap();

        Self {
            ferris_borrow_fail: ferris_borrow_angry,
            screen: Screen::Menu,
            game: None,
            velocity: 0.0,
            ferris_planet,
            consolas: graphics::Font::new(ctx, "/Consolas.ttf").unwrap(),
            ferris_dead: Source::new(ctx, "/dead.mp3").unwrap()
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.screen {
            Screen::Menu => menu::draw(self, ctx),
            Screen::Play => {
                self.velocity += 0.7;
                self.game.as_mut().unwrap().lock().unwrap().pos_y += self.velocity as f32;

                if self.game.as_ref().unwrap().lock().unwrap().pos_y > HEIGHT {
                    self.screen = Screen::Dead;
                    self.ferris_dead.play().unwrap();
                }
                
                let game_state = self.game.as_ref().unwrap();

                game::draw(&game_state.lock().unwrap(), ctx)
            },
            Screen::Dead => {
                dead::draw(self, ctx)
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.screen {
            Screen::Menu => menu::draw(self, ctx),
            Screen::Play => {
                let game_state = self.game.as_ref().unwrap();

                game::draw(&game_state.lock().unwrap(), ctx)
            },
            Screen::Dead => {
                dead::draw(self, ctx)
            }
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
                if keycode == KeyCode::Key8 {
                    exit(0);
                } else if keycode == KeyCode::Key7 {
                    self.screen = Screen::Play;
                    self.game = Some(Mutex::new(
                        game::Game { 
                            ferris_borrow_fail: self.ferris_borrow_fail.to_owned(),
                            pos_y: HEIGHT / 2.0
                        }
                    ));
                }
            }
            Screen::Play => {
                if keycode == KeyCode::Space {
                   self.velocity += -12.0;
                }
            },
            Screen::Dead => {

            }
        }
    }
}
