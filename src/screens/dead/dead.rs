use std::rc::Rc;

use ggez::{
    graphics::Color,
    graphics::{self, Scale, Text, TextFragment},
    nalgebra::Point2,
    Context, GameResult,
};
use graphics::DrawParam;

use crate::{utils::AssetManager, WIDTH};

pub struct Death {
    asset_manager: Rc<AssetManager>,
}

impl Death {
    pub fn spawn(_ctx: &mut Context, asset_manager: Rc<AssetManager>) -> Self {
        Self { asset_manager }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let consolas = self.asset_manager.get_font("Consolas.ttf");
        let ferris_planet = self.asset_manager.get_image("ferris_planet.png");

        let dead = Text::new(
            TextFragment::new("YOU DEAD")
                .scale(Scale::uniform(35.0))
                .font(consolas)
                .color(Color::from_rgb(255, 80, 76)),
        );

        let unsafe_dead = Text::new(
            TextFragment::new("unsafe")
                .scale(Scale::uniform(30.0))
                .font(consolas)
                .color(Color::from_rgb(74, 129, 191)),
        );

        let unsafe_dead_block_start = Text::new(
            TextFragment::new("{")
                .scale(Scale::uniform(30.0))
                .font(consolas)
                .color(Color::from_rgb(255, 255, 255)),
        );

        let unsafe_dead_block_func = Text::new(
            TextFragment::new("dead()")
                .scale(Scale::uniform(30.0))
                .font(consolas)
                .color(Color::from_rgb(214, 208, 132)),
        );

        let unsafe_dead_block_end = Text::new(
            TextFragment::new("}")
                .scale(Scale::uniform(30.0))
                .font(consolas)
                .color(Color::from_rgb(255, 255, 255)),
        );

        graphics::draw(
            ctx,
            &dead,
            DrawParam::default().dest(Point2::new((WIDTH / 2.0) - 60.0, 40.0)),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead,
            DrawParam::default().dest(Point2::new((WIDTH / 2.0) - 200.0, 200.0)),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead_block_start,
            DrawParam::default().dest(Point2::new((WIDTH / 2.0) - 90.0, 200.0)),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead_block_func,
            DrawParam::default().dest(Point2::new((WIDTH / 2.0) - 125.0, 260.0)),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead_block_end,
            DrawParam::default().dest(Point2::new((WIDTH / 2.0) - 200.0, 300.0)),
        )?;

        graphics::draw(
            ctx,
            &ferris_planet,
            DrawParam::default().dest(Point2::new((WIDTH / 2.0) - 10.0, 240.0)),
        )?;

        graphics::present(ctx)
    }

    pub fn update(&self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
