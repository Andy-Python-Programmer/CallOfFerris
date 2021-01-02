use ggez::{
    nalgebra::{distance, Point2},
    Context, GameResult,
};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::utils::{AssetManager, Position};

use super::enemy::Enemy;

pub enum PlayerWeapon {
    Turbofish(Turbofish),
    Grappling(Grappling),
}

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

pub struct Grappling {
    position: Position,
    traveled_count: i32,
}

impl Grappling {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        _asset_manager: &AssetManager,
        enemies: &Vec<Enemy>,
    ) -> Option<Self> {
        let mut position = None;

        for enemy in enemies {
            if pos_x - enemy.position().pos_start.x < 100.0
                && pos_x - enemy.position().pos_start.x > -300.0
            {
                position = Some(Position::new(
                    pos_x,
                    pos_y,
                    distance(
                        &Point2::new(pos_x, 0.0),
                        &Point2::new(enemy.position().pos_start.x, 0.0),
                    ) as u16,
                    10,
                ));
            }
        }

        if position.is_none() {
            return None;
        }

        Some(Self {
            position: position.unwrap(),
            traveled_count: 0,
        })
    }

    pub fn draw(
        &mut self,
        _ctx: &mut Context,
        _camera: &Camera,
        _asset_manager: &AssetManager,
    ) -> GameResult<()> {
        // TODO

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

    pub fn _position(&self) -> Position {
        self.position
    }
}
