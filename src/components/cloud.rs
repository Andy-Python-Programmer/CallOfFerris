use ggez::{
    graphics::{self, Image},
    mint::{Point2, Vector2},
    Context, GameResult,
};
use graphics::DrawParam;

use crate::WIDTH;

pub struct Cloud {
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    speed: f32,
}

impl Cloud {
    pub fn new(pos_x: f32, pos_y: f32, scale: f32, speed: f32) -> Self {
        Self {
            pos_x,
            pos_y,
            scale,
            speed
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, resources: &Vec<Image>) -> GameResult<()> {
        graphics::draw(
            ctx,
            &resources[0],
            DrawParam::default()
                .scale(Vector2 {
                    x: self.scale,
                    y: self.scale,
                })
                .dest(Point2 {
                    x: self.pos_x,
                    y: self.pos_y,
                }),
        )?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        self.pos_x += delta_time * self.speed;
        
        if self.pos_x > WIDTH + 100. {
            self.pos_x = -100.;
        }
    }
}
