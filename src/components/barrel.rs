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

pub struct Barrel {
    position: Position,
}

impl Barrel {
    pub fn new(pos_x: f32, asset_manager: &AssetManager) -> Self {
        let barrel = asset_manager.get_image("Some(barrel).png");

        let position = Position::new(
            pos_x,
            -HEIGHT2 + (barrel.height() + 40) as f32,
            barrel.width(),
            barrel.height(),
        );

        Self { position }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let barrel = asset_manager.get_image("Some(barrel).png");

        barrel.draw_camera(
            camera,
            ctx,
            Vec2::new(self.position.pos_start.x, self.position.pos_start.y),
            0.,
        )?;

        Ok(())
    }

    pub fn position(&self) -> Position {
        self.position
    }
}
