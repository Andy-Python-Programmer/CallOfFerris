use ggez::{
    graphics,
    mint::{Point2, Vector2},
    Context, GameResult,
};
use graphics::DrawParam;

use crate::{utils::AssetManager, WIDTH};

use nphysics2d::nalgebra as na;

pub struct Cloud {
    position: na::Point2<f32>,

    scale: f32,
    speed: f32,
}

impl Cloud {
    pub fn new(pos_x: f32, pos_y: f32, scale: f32, speed: f32) -> Self {
        let position = na::Point2::new(pos_x, pos_y);

        Self {
            position,
            scale,
            speed,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult<()> {
        let cloud = asset_manager.get_image("Some(cloud).png");

        graphics::draw(
            ctx,
            &cloud,
            DrawParam::default()
                .scale(Vector2 {
                    x: self.scale,
                    y: self.scale,
                })
                .dest(Point2 {
                    x: self.position.x,
                    y: self.position.y,
                }),
        )?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        self.position.x += delta_time * self.speed;

        if self.position.x > WIDTH + 100. {
            self.position = na::Point2::new(-100., self.position.y);
        }
    }
}
