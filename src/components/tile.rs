use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::DrawParam;
use nphysics2d::nalgebra as na;
use nphysics2d::object::DefaultBodyHandle;

use crate::{
    physics::{isometry_to_point, Physics},
    utils::AssetManager,
    HEIGHT,
};

const HEIGHT2: f32 = HEIGHT / 2.;

pub enum TileType {
    LEFT,
    CENTER,
    RIGHT,
}

pub struct Tile {
    width: f32,
    height: f32,

    body: DefaultBodyHandle,
    tile_type: TileType,
}

impl Tile {
    pub fn new(
        pos_x: f32,
        physics: &mut Physics,
        asset_manager: &AssetManager,
        tile_type: TileType,
    ) -> Self {
        let width;
        let height;

        let pos_y = HEIGHT2 - 64.0;

        match tile_type {
            TileType::LEFT => {
                let ground_left = asset_manager.get_image("ground_left.png");

                width = ground_left.width();
                height = ground_left.height();
            }
            TileType::CENTER => {
                let ground_centre = asset_manager.get_image("ground_centre.png");

                width = ground_centre.width();
                height = ground_centre.height();
            }
            TileType::RIGHT => {
                let ground_right = asset_manager.get_image("ground_right.png");

                width = ground_right.width();
                height = ground_right.height();
            }
        }

        let body = physics.create_tile(na::Point2::new(pos_x, pos_y), width, height);

        Self {
            tile_type,
            body,

            width: width as f32,
            height: height as f32,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        physics: &mut Physics,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let ground_left = asset_manager.get_image("ground_left.png");
        let ground_centre = asset_manager.get_image("ground_centre.png");
        let ground_right = asset_manager.get_image("ground_right.png");

        let ground_position = self.position(physics);
        let tile_position =
            camera.calculate_dest_point(Vec2::new(ground_position.x, ground_position.y));

        match self.tile_type {
            TileType::LEFT => {
                graphics::draw(
                    ctx,
                    &ground_left,
                    DrawParam::default()
                        .dest(Point2::new(tile_position.x, tile_position.y))
                        .offset(Point2::new(0.5, 0.5)),
                )?;
            }

            TileType::CENTER => {
                graphics::draw(
                    ctx,
                    &ground_centre,
                    DrawParam::default()
                        .dest(Point2::new(tile_position.x, tile_position.y))
                        .offset(Point2::new(0.5, 0.5)),
                )?;
            }
            TileType::RIGHT => {
                graphics::draw(
                    ctx,
                    &ground_right,
                    DrawParam::default()
                        .dest(Point2::new(tile_position.x, tile_position.y))
                        .offset(Point2::new(0.5, 0.5)),
                )?;
            }
        }

        Ok(())
    }

    pub fn position(&self, physics: &mut Physics) -> na::Point2<f32> {
        let ground_body = physics.get_rigid_body(self.body);
        let ground_position = isometry_to_point(ground_body.position());

        ground_position
    }

    pub fn dimensions(&self) -> na::Point2<f32> {
        na::Point2::new(self.width, self.height)
    }
}
