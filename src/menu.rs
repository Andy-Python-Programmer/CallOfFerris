use ggez::{
    event::KeyCode,
    graphics::{self, Scale, Text, TextFragment},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};
use graphics::DrawParam;
use std::process::exit;

use crate::HEIGHT;
use crate::WIDTH;

pub struct Menu {
    pub consolas: graphics::Font,
    pub ferris_ninja: graphics::Image,
    pub logo: graphics::Image,
    pub bg: graphics::Image,
}

impl Menu {
    pub fn create(ctx: &mut Context) -> Self {
        Self {
            consolas: graphics::Font::new(ctx, "/fonts/Consolas.ttf").unwrap(),
            ferris_ninja: graphics::Image::new(ctx, "/images/ferris_ninja.png").unwrap(),
            logo: graphics::Image::new(ctx, "/images/logo.png").unwrap(),
            bg: graphics::Image::new(ctx, "/images/menu_bg.png").unwrap(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen
        graphics::clear(ctx, graphics::BLACK);

        graphics::draw(
            ctx,
            &self.bg,
            DrawParam::default()
                .dest(Point2::new(0.0, 0.0))
                .scale(Vector2::new(0.6, 0.5)),
        )?;

        graphics::draw(
            ctx,
            &self.logo,
            (ggez::nalgebra::Point2::new(
                WIDTH - (self.logo.width() as f32 + 20.0),
                10.0,
            ),),
        )?;

        graphics::draw(
            ctx,
            &self.ferris_ninja,
            (ggez::nalgebra::Point2::new(
                WIDTH - (WIDTH - 400.0),
                HEIGHT - (&self.ferris_ninja.height() + 140) as f32,
            ),),
        )?;

        let press_and_to = TextFragment {
            text: "Press & to".to_owned(),
            font: Some(self.consolas),
            scale: Some(Scale::uniform(15.0)),

            ..Default::default()
        };

        let press_pointer_to = TextFragment {
            text: "Press * to".to_owned(),
            font: Some(self.consolas),
            scale: Some(Scale::uniform(15.0)),

            ..Default::default()
        };

        graphics::draw(
            ctx,
            &Text::new(press_and_to),
            (ggez::nalgebra::Point2::new(
                WIDTH - 200.0,
                HEIGHT - (&self.ferris_ninja.height() + 30) as f32,
            ),),
        )?;

        let play_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                WIDTH - 200.0,
                HEIGHT - (&self.ferris_ninja.height() + 10) as f32,
                220.0,
                40.0,
            ),
            [36.0 / 255.0, 36.0 / 255.0, 36.0 / 255.0, 0.5].into(),
        )?;

        let quit_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                WIDTH - 200.0,
                HEIGHT - (&self.ferris_ninja.height() - 70) as f32,
                220.0,
                40.0,
            ),
            [36.0 / 255.0, 36.0 / 255.0, 36.0 / 255.0, 0.5].into(),
        )?;

        let play_text = TextFragment {
            text: "PLAY".to_owned(),
            font: Some(self.consolas),
            scale: Some(Scale::uniform(20.0)),

            ..Default::default()
        };

        let quit_text = TextFragment {
            text: "QUIT".to_owned(),
            font: Some(self.consolas),
            scale: Some(Scale::uniform(20.0)),

            ..Default::default()
        };

        graphics::draw(ctx, &play_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &quit_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        graphics::draw(
            ctx,
            &Text::new(play_text),
            (ggez::nalgebra::Point2::new(
                WIDTH - 170.0,
                HEIGHT - self.ferris_ninja.height() as f32,
            ),),
        )?;

        graphics::draw(
            ctx,
            &Text::new(press_pointer_to),
            (ggez::nalgebra::Point2::new(
                WIDTH - 200.0,
                HEIGHT - (&self.ferris_ninja.height() - 50) as f32,
            ),),
        )?;

        graphics::draw(
            ctx,
            &Text::new(quit_text),
            (ggez::nalgebra::Point2::new(
                WIDTH - 170.0,
                HEIGHT - (&self.ferris_ninja.height() - 80) as f32,
            ),),
        )?;

        graphics::present(ctx)
    }

    pub fn update(&self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    pub fn key_press(&self, keycode: KeyCode) -> Option<crate::Screen> {
        if keycode == KeyCode::Key7 {
            return Some(crate::Screen::Play);
        } else if keycode == KeyCode::Key8 {
            exit(0);
        }

        return None;
    }
}
