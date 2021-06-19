use ggez::{
    event::KeyCode,
    graphics::{self, Scale, Text, TextFragment},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};
use graphics::{Color, DrawParam};
use std::{process::exit, rc::Rc};

use crate::utils::AssetManager;
use crate::Screen;

pub struct Menu {
    asset_manager: Rc<AssetManager>,
}

impl Menu {
    pub fn create(_ctx: &mut Context, asset_manager: Rc<AssetManager>) -> Self {
        Self { asset_manager }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let (width, height) = graphics::drawable_size(ctx);

        let logo = self.asset_manager.get_image("logo.png");
        let ferris_ninja = self.asset_manager.get_image("ferris_ninja.png");
        let menu_bg = self.asset_manager.get_image("menu_bg.png");

        let consolas = self.asset_manager.get_font("Consolas.ttf");

        // Clear the screen
        graphics::clear(ctx, graphics::BLACK);

        graphics::draw(
            ctx,
            &menu_bg,
            DrawParam::default().scale(Vector2::new(0.6, 0.5)),
        )?;

        graphics::draw(
            ctx,
            &logo,
            DrawParam::default().dest(Point2::new(width - (logo.width() as f32 + 20.0), 10.0)),
        )?;

        graphics::draw(
            ctx,
            &ferris_ninja,
            DrawParam::default().dest(Point2::new(
                width - (width - 400.0),
                height - (ferris_ninja.height() + 140) as f32,
            )),
        )?;

        let press_and_to = TextFragment::new("Press & to")
            .font(consolas)
            .scale(Scale::uniform(15.0));

        let press_pointer_to = TextFragment::new("Press * to")
            .font(consolas)
            .scale(Scale::uniform(15.0));

        graphics::draw(
            ctx,
            &Text::new(press_and_to),
            DrawParam::default().dest(Point2::new(
                width - 200.0,
                height - (ferris_ninja.height() + 30) as f32,
            )),
        )?;

        let play_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                width - 200.0,
                height - (ferris_ninja.height() + 10) as f32,
                220.0,
                40.0,
            ),
            Color::from_rgba(36, 36, 36, 128),
        )?;

        let quit_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                width - 200.0,
                height - (ferris_ninja.height() - 70) as f32,
                220.0,
                40.0,
            ),
            Color::from_rgba(36, 36, 36, 128),
        )?;

        let play_text = TextFragment::new("PLAY")
            .font(consolas)
            .scale(Scale::uniform(20.0));

        let quit_text = TextFragment::new("QUIT")
            .font(consolas)
            .scale(Scale::uniform(20.0));

        graphics::draw(ctx, &play_rect, DrawParam::default())?;
        graphics::draw(ctx, &quit_rect, DrawParam::default())?;

        graphics::draw(
            ctx,
            &Text::new(play_text),
            DrawParam::default().dest(Point2::new(
                width - 170.0,
                height - ferris_ninja.height() as f32,
            )),
        )?;

        graphics::draw(
            ctx,
            &Text::new(press_pointer_to),
            DrawParam::default().dest(Point2::new(
                width - 200.0,
                height - (ferris_ninja.height() - 50) as f32,
            )),
        )?;

        graphics::draw(
            ctx,
            &Text::new(quit_text),
            DrawParam::default().dest(Point2::new(
                width - 170.0,
                height - (ferris_ninja.height() - 80) as f32,
            )),
        )?;

        graphics::present(ctx)
    }

    pub fn update(&self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    pub fn key_press(&self, keycode: KeyCode) -> Option<Screen> {
        if keycode == KeyCode::Key7 {
            return Some(Screen::Play);
        } else if keycode == KeyCode::Key8 {
            exit(0);
        }

        None
    }
}
