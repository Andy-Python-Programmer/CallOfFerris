use ggez::{Context, GameResult, graphics::Image};
use ggez_goodies::{camera::{Camera, CameraDraw}, nalgebra_glm::Vec2};

use crate::HEIGHT;

pub struct Barrel {
    pub pos_x: f32
}

impl Barrel {
    pub fn new(pos_x: f32) -> Self {
        Self {
            pos_x
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        resources: &Vec<Image>,
    ) -> GameResult<()> {
        const HEIGHT2: f32 = HEIGHT / 2.;


        &resources[0].draw_camera(
            camera,
            ctx,
            Vec2::new(self.pos_x, -HEIGHT2 + (resources[0].height() + 40) as f32),
            0.
        );
        
        Ok(())
    }
}