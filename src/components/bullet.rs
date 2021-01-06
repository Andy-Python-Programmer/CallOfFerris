use ggez::{
    graphics::{self, DrawParam},
    nalgebra::Point2,
    Context, GameResult,
};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};

use nphysics2d::{algebra::Velocity2, nalgebra as na, object::DefaultBodyHandle};
use physics::ObjectData;

use crate::{
    physics::{self, isometry_to_point, Physics},
    utils::{AssetManager, Position},
};

use super::enemy::Enemy;

pub enum PlayerWeapon {
    Turbofish(Turbofish),
    Grappling(Grappling),
}

pub struct Turbofish {
    body: DefaultBodyHandle,
}

impl Turbofish {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        physics: &mut Physics,
        asset_manager: &AssetManager,
    ) -> Self {
        let turbofish_bullet = asset_manager.get_image("Some(turbofish).png");
        let body = physics.create_bullet(
            na::Point2::new(pos_x, pos_y),
            turbofish_bullet.width(),
            turbofish_bullet.height(),
        );

        let bullet_body = physics.get_rigid_body_mut(body);
        bullet_body.set_velocity(Velocity2::linear(1000.0, 0.0));

        Self { body }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        physics: &mut Physics,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let turbofish_bullet = asset_manager.get_image("Some(turbofish).png");

        let bullet_position = self.position(physics);
        let turbofish_position =
            camera.calculate_dest_point(Vec2::new(bullet_position.x, bullet_position.y));

        graphics::draw(
            ctx,
            &turbofish_bullet,
            DrawParam::default()
                .dest(Point2::new(turbofish_position.x, turbofish_position.y))
                .offset(Point2::new(0.5, 0.5)),
        )?;

        Ok(())
    }

    pub fn update(&mut self, physics: &mut Physics) -> bool {
        for collision in physics.collisions(self.body) {
            if collision.0 .1 == ObjectData::Ground {
                return true;
            }
        }

        false
    }

    pub fn is_touching(&mut self, physics: &mut Physics, handle: DefaultBodyHandle) -> bool {
        for collision in physics.collisions(self.body) {
            if collision.1 == handle {
                return true;
            }
        }

        false
    }

    pub fn destroy(&mut self, physics: &mut Physics) {
        physics.destroy_body(self.body);
    }

    pub fn position(&self, physics: &mut Physics) -> na::Point2<f32> {
        let bullet_body = physics.get_rigid_body_mut(self.body);
        let bullet_position = isometry_to_point(bullet_body.position());

        bullet_position
    }
}

pub struct Grappling {
    position: Position,
    traveled_count: i32,
}

impl Grappling {
    pub fn new(
        _pos_x: f32,
        _pos_y: f32,
        _asset_manager: &AssetManager,
        _enemies: &Vec<Enemy>,
    ) -> Option<Self> {
        // FIXME
        // let mut position = None;

        // for enemy in enemies {
        //     if pos_x - enemy.position().pos_start.x < 100.0
        //         && pos_x - enemy.position().pos_start.x > -300.0
        //     {
        //         position = Some(Position::new(
        //             pos_x,
        //             pos_y,
        //             distance(
        //                 &Point2::new(pos_x, 0.0),
        //                 &Point2::new(enemy.position().pos_start.x, 0.0),
        //             ) as u16,
        //             10,
        //         ));
        //     }
        // }

        // if position.is_none() {
        //     return None;
        // }

        // Some(Self {
        //     position: position.unwrap(),
        //     traveled_count: 0,
        // })

        None
    }

    pub fn draw(
        &mut self,
        _ctx: &mut Context,
        _camera: &Camera,
        _asset_manager: &AssetManager,
    ) -> GameResult<()> {
        // FIXME

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
