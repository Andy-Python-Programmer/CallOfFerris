use ggez::{Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::{utils::AssetManager, HEIGHT};

const HEIGHT2: f32 = HEIGHT / 2.;

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
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let ground_left = asset_manager.get_image("ground_left.png");
        let ground_centre = asset_manager.get_image("ground_centre.png");
        let ground_right = asset_manager.get_image("ground_right.png");

        match self.tile_type {
            TileType::LEFT => {
                ground_left.draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.pos_x, -HEIGHT2 + ground_left.height() as f32),
                    0.0,
                )?;
            }

            TileType::CENTER => {
                ground_centre.draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.pos_x, -HEIGHT2 + ground_centre.height() as f32),
                    0.0,
                )?;
            }
            TileType::RIGHT => {
                ground_right.draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.pos_x, -HEIGHT2 + ground_right.height() as f32),
                    0.0,
                )?;
            }
        }

        Ok(())
    }
}
