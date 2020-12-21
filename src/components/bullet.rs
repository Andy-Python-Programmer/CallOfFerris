use ggez::{graphics::Image, Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

pub struct Turbofish {
    pub pos_x: f32,
    pub pos_y: f32,

    traveled_count: i32,
}

impl Turbofish {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            pos_x,
            pos_y,
            traveled_count: 0,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        resources: &Vec<Image>,
    ) -> GameResult<()> {
        &resources[0].draw_camera(camera, ctx, Vec2::new(self.pos_x, self.pos_y), 1.5708);

        Ok(())
    }

    pub fn go_boom(&mut self) -> bool {
        if self.traveled_count < 100 {
            self.traveled_count += 1;
            self.pos_x += 10.;

            false
        } else {
            true
        }
    }
}
