use ggez::{Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::utils::{AssetManager, Position};

pub struct Turbofish {
    position: Position,
    traveled_count: i32,
}

impl Turbofish {
    pub fn new(pos_x: f32, pos_y: f32, asset_manager: &AssetManager) -> Self {
        let turbofish_bullet = asset_manager.get_image("Some(turbofish).png");
        let position = Position::new(
            pos_x,
            pos_y,
            turbofish_bullet.width(),
            turbofish_bullet.height(),
        );

        Self {
            position,
            traveled_count: 0,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let turbofish_bullet = asset_manager.get_image("Some(turbofish).png");

        turbofish_bullet.draw_camera(
            camera,
            ctx,
            Vec2::new(self.position.pos_start.x, self.position.pos_start.y),
            1.5708,
        )?;

        Ok(())
    }

    pub fn go_boom(&mut self) -> bool {
        if self.traveled_count < 100 {
            self.traveled_count += 1;
            self.position.move_by("x+", 10.0);

            false
        } else {
            true
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}
