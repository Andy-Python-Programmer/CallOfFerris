use ggez::{
    graphics::{self, DrawParam},
    nalgebra::Point2,
    Context, GameResult,
};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};

use nphysics2d::{algebra::Velocity2, math::Velocity, nalgebra as na, object::DefaultBodyHandle};
use physics::ObjectData;

use crate::{
    physics::{self, isometry_to_point, Physics},
    utils::AssetManager,
};

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
    grapple_to: DefaultBodyHandle,
    player_body: DefaultBodyHandle,
}

impl Grappling {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        physics: &mut Physics,
        handle: DefaultBodyHandle,
    ) -> Option<Self> {
        let ray_cast = physics.ray_cast(na::Point2::new(pos_x, pos_y), na::Vector2::new(1.0, 1.0));

        if ray_cast.len() > 0 {
            for object in ray_cast {
                if object.0 == ObjectData::Barrel {
                    let body = object.1.body();
                    let body_pos = isometry_to_point(physics.get_rigid_body(body).position());

                    physics
                        .get_rigid_body_mut(body)
                        .set_velocity(Velocity::linear(pos_x - body_pos.x, pos_y - body_pos.y));

                    return Some(Self {
                        grapple_to: body,
                        player_body: handle,
                    });
                }
            }

            None
        } else {
            None
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        physics: &mut Physics,
    ) -> GameResult<()> {
        // FIXME

        let player = isometry_to_point(physics.get_rigid_body(self.player_body).position());

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.0,
                0.0,
                physics.distance(self.player_body, self.grapple_to),
                10.0,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        let pos = camera.calculate_dest_point(Vec2::new(player.x + 140.0, player.y));

        graphics::draw(
            ctx,
            &rect,
            DrawParam::default().dest(Point2::new(pos.x, pos.y)),
        )?;

        Ok(())
    }

    pub fn update(&mut self, physics: &mut Physics) {
        let player = isometry_to_point(physics.get_rigid_body(self.player_body).position());
        let object = isometry_to_point(physics.get_rigid_body(self.grapple_to).position());

        if physics.distance(self.player_body, self.grapple_to) as i32 > 1 {
            physics
                .get_rigid_body_mut(self.grapple_to)
                .set_velocity(Velocity::linear(player.x - object.x, player.y - object.y));
        }
    }
}
