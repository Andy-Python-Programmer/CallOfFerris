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

use super::player::Player;

pub struct Enemy {
    position: Position,
}

impl Enemy {
    pub fn new(pos_x: f32, asset_manager: &AssetManager) -> Self {
        let gopher = asset_manager.get_image("gopher.png");

        let position = Position::new(pos_x, -HEIGHT2 + 190., gopher.width(), gopher.height());

        Self { position }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let gopher = asset_manager.get_image("gopher.png");
        let gun = asset_manager.get_image("Some(gun).png");

        gopher.draw_camera(
            &camera,
            ctx,
            Vec2::new(self.position.pos_start.x, self.position.pos_start.y),
            0.0,
        )?;

        gun.draw_camera(
            &camera,
            ctx,
            Vec2::new(self.position.pos_start.x - 50., -HEIGHT2 + 150.),
            0.0,
        )?;

        Ok(())
    }

    pub fn update(&mut self, player: &Player) {
        // Can the enemy see the player?
        if player.pos_x - self.position.pos_start.x > -457.0
            && player.pos_x - self.position.pos_start.x < 457.0
        {
            // TODO: The enemy shoots the player as soon as it see's the player.
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}
