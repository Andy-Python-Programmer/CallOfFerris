use ggez::{graphics, Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};
use graphics::Image;

use crate::HEIGHT;

pub struct Enemy {
    pub pos_x: f32,
}

impl Enemy {
    pub fn new(pos_x: f32) -> Self {
        Self { pos_x }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        resources: &Vec<Image>,
    ) -> GameResult<()> {
        const HEIGHT2: f32 = HEIGHT / 2.;

        &resources[0].draw_camera(&camera, ctx, Vec2::new(self.pos_x, -HEIGHT2 + 190.), 0.0);

        &resources[1].draw_camera(
            &camera,
            ctx,
            Vec2::new(self.pos_x - 50., -HEIGHT2 + 150.),
            0.0,
        );

        Ok(())
    }
}
