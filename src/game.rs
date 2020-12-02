use std::sync::Mutex;

use ggez::{Context, GameResult, audio::Source, event::KeyCode, audio::SoundSource, graphics};

use crate::WIDTH;
use crate::HEIGHT;

pub struct Game {
    pub ferris_ninja: graphics::Image,
    pub ferris_death_audio: Source, 
    pub pos_y: f32,
}

impl Game {
    pub fn create(ctx: &mut Context) -> Mutex<Self> {
        Mutex::new(
            Self {
                ferris_ninja: graphics::Image::new(ctx, "/ferris_ninja.png").unwrap(),
                ferris_death_audio: Source::new(ctx, "/dead.mp3").unwrap(),
                pos_y: 10.0
            }
        )
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        graphics::draw(
            ctx,
            &self.ferris_ninja,
            (ggez::nalgebra::Point2::new(
                (WIDTH / 2.0) - 80.0,
                self.pos_y,
            ),),
        )
        .unwrap();

        graphics::present(ctx)
    }

    pub fn update(&self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    pub fn key_press(&mut self, _keycode: KeyCode) -> Option<crate::Screen> {
        self.ferris_death_audio.play().expect("Cannot play the sad violin.");

        None
    }
}
