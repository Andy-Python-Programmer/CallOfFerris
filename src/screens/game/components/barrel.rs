use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::DrawParam;

use crate::{
    game::physics::{isometry_to_point, Physics},
    utils::AssetManager,
    HEIGHT,
};

const HEIGHT2: f32 = HEIGHT / 2.;

use nphysics2d::{nalgebra as na, object::DefaultBodyHandle};

pub struct Barrel {
    body: DefaultBodyHandle,
}

impl Barrel {
    pub fn new(pos_x: f32, physics: &mut Physics, asset_manager: &AssetManager) -> Self {
        let barrel = asset_manager.get_image("Some(barrel).png");

        let body = physics.create_barrel(
            na::Point2::new(pos_x, HEIGHT2 - 155.0),
            barrel.width(),
            barrel.height(),
        );

        Self { body }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        physics: &mut Physics,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let barrel = asset_manager.get_image("Some(barrel).png");

        let barrel_position = self.position(physics);
        let barrel_pos_camera =
            camera.calculate_dest_point(Vec2::new(barrel_position.x, barrel_position.y));

        graphics::draw(
            ctx,
            &barrel,
            DrawParam::default()
                .dest(Point2::new(barrel_pos_camera.x, barrel_pos_camera.y))
                .offset(Point2::new(0.5, 0.5)),
        )?;

        Ok(())
    }

    pub fn position(&self, physics: &mut Physics) -> na::Point2<f32> {
        let barrel_body = physics.get_rigid_body_mut(self.body);
        let barrel_position = isometry_to_point(barrel_body.position());

        barrel_position
    }

    pub fn handle(&self) -> DefaultBodyHandle {
        self.body
    }

    pub fn destroy(&self, physics: &mut Physics) {
        physics.destroy_body(self.body);
    }
}
