use ggez::{Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::{
    utils::{AssetManager, Position},
    HEIGHT,
};

const HEIGHT2: f32 = HEIGHT / 2.;

pub enum TileType {
    LEFT,
    CENTER,
    RIGHT,
}

pub struct Tile {
    position: Position,
    width: f32,

    tile_type: TileType,
}

impl Tile {
    pub fn new(pos_x: f32, asset_manager: &AssetManager, tile_type: TileType) -> Self {
        let position;
        let width;

        match tile_type {
            TileType::LEFT => {
                let ground_left = asset_manager.get_image("ground_left.png");

                position = Position::new(
                    pos_x,
                    -HEIGHT2 + ground_left.height() as f32,
                    ground_left.width(),
                    ground_left.height(),
                );

                width = ground_left.width();
            }
            TileType::CENTER => {
                let ground_centre = asset_manager.get_image("ground_centre.png");

                position = Position::new(
                    pos_x,
                    -HEIGHT2 + ground_centre.height() as f32,
                    ground_centre.width(),
                    ground_centre.height(),
                );

                width = ground_centre.width();
            }
            TileType::RIGHT => {
                let ground_right = asset_manager.get_image("ground_right.png");

                position = Position::new(
                    pos_x,
                    -HEIGHT2 + ground_right.height() as f32,
                    ground_right.width(),
                    ground_right.height(),
                );

                width = ground_right.width();
            }
        }

        Self {
            position,
            tile_type,
            width: width as f32,
        }
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
                    Vec2::new(self.position.pos_start.x, self.position.pos_start.y),
                    0.0,
                )?;
            }

            TileType::CENTER => {
                ground_centre.draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.position.pos_start.x, self.position.pos_start.y),
                    0.0,
                )?;
            }
            TileType::RIGHT => {
                ground_right.draw_camera(
                    &camera,
                    ctx,
                    Vec2::new(self.position.pos_start.x, self.position.pos_start.y),
                    0.0,
                )?;
            }
        }

        Ok(())
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}
