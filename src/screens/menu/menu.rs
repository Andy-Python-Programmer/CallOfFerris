use ggez::{
    event::KeyCode,
    graphics::{self, Scale, Text, TextFragment},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};
use graphics::DrawParam;
use std::{process::exit, rc::Rc};

use crate::WIDTH;
use crate::{utils::AssetManager, HEIGHT};

pub struct Menu {
    asset_manager: Rc<AssetManager>,
}

impl Menu {
    pub fn create(_ctx: &mut Context, asset_manager: Rc<AssetManager>) -> Self {
        Self { asset_manager }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let logo = self.asset_manager.get_image("logo.png");
        let ferris_ninja = self.asset_manager.get_image("ferris_ninja.png");
        let menu_bg = self.asset_manager.get_image("menu_bg.png");

        let consolas = self.asset_manager.get_font("Consolas.ttf");

        // Clear the screen
        graphics::clear(ctx, graphics::BLACK);

        graphics::draw(
            ctx,
            &menu_bg,
            DrawParam::default()
                .dest(Point2::new(0.0, 0.0))
                .scale(Vector2::new(0.6, 0.5)),
        )?;

        graphics::draw(
            ctx,
            &logo,
            (ggez::nalgebra::Point2::new(
                WIDTH - (logo.width() as f32 + 20.0),
                10.0,
            ),),
        )?;

        graphics::draw(
            ctx,
            &ferris_ninja,
            (ggez::nalgebra::Point2::new(
                WIDTH - (WIDTH - 400.0),
                HEIGHT - (ferris_ninja.height() + 140) as f32,
            ),),
        )?;

        let press_and_to = TextFragment {
            text: "Press & to".to_owned(),
            font: Some(consolas),
            scale: Some(Scale::uniform(15.0)),

            ..Default::default()
        };

        let press_pointer_to = TextFragment {
            text: "Press * to".to_owned(),
            font: Some(consolas),
            scale: Some(Scale::uniform(15.0)),

            ..Default::default()
        };

        graphics::draw(
            ctx,
            &Text::new(press_and_to),
            (ggez::nalgebra::Point2::new(
                WIDTH - 200.0,
                HEIGHT - (ferris_ninja.height() + 30) as f32,
            ),),
        )?;

        let play_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                WIDTH - 200.0,
                HEIGHT - (ferris_ninja.height() + 10) as f32,
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
                HEIGHT - (ferris_ninja.height() - 70) as f32,
                220.0,
                40.0,
            ),
            [36.0 / 255.0, 36.0 / 255.0, 36.0 / 255.0, 0.5].into(),
        )?;

        let play_text = TextFragment {
            text: "PLAY".to_owned(),
            font: Some(consolas),
            scale: Some(Scale::uniform(20.0)),

            ..Default::default()
        };

        let quit_text = TextFragment {
            text: "QUIT".to_owned(),
            font: Some(consolas),
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
                HEIGHT - ferris_ninja.height() as f32,
            ),),
        )?;

        graphics::draw(
            ctx,
            &Text::new(press_pointer_to),
            (ggez::nalgebra::Point2::new(
                WIDTH - 200.0,
                HEIGHT - (ferris_ninja.height() - 50) as f32,
            ),),
        )?;

        graphics::draw(
            ctx,
            &Text::new(quit_text),
            (ggez::nalgebra::Point2::new(
                WIDTH - 170.0,
                HEIGHT - (ferris_ninja.height() - 80) as f32,
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
