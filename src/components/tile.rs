use ggez::{graphics::Image, Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::HEIGHT;

pub enum TileType {
    LEFT,
    CENTER,
    RIGHT,
}

pub struct Tile {
    pub pos_x: f32,
    tile_type: TileType,
}

impl Tile {
    pub fn new(pos_x: f32, tile_type: TileType) -> Self {
        Self { pos_x, tile_type }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        resources: &Vec<Image>,
    ) -> GameResult<()> {
        const HEIGHT2: f32 = HEIGHT / 2.;

        match self.tile_type {
            TileType::LEFT => {
                &resources[0].draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.pos_x, -HEIGHT2 + resources[0].height() as f32),
                    0.0,
                );
            }

            TileType::CENTER => {
                &resources[1].draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.pos_x, -HEIGHT2 + resources[0].height() as f32),
                    0.0,
                );
            }
            TileType::RIGHT => {
                &resources[2].draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.pos_x, -HEIGHT2 + resources[0].height() as f32),
                    0.0,
                );
            }
        }

        Ok(())
    }
}
