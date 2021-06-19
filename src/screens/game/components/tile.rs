use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::DrawParam;
use nphysics2d::nalgebra as na;
use nphysics2d::object::DefaultBodyHandle;

use crate::{
    game::physics::{isometry_to_point, Physics},
    utils::AssetManager,
};

pub enum TileType {
    Left,
    Center,
    Right,
}

pub struct Tile {
    width: f32,
    height: f32,

    body: DefaultBodyHandle,
    tile_type: TileType,
}

impl Tile {
    pub fn new(
        ctx: &mut Context,
        pos_x: f32,
        physics: &mut Physics,
        asset_manager: &AssetManager,
        tile_type: TileType,
    ) -> Self {
        let (_, height) = graphics::drawable_size(ctx);

        let tile_width;
        let tile_height;

        let pos_y = height / 2.0 - 64.0;

        match tile_type {
            TileType::Left => {
                let ground_left = asset_manager.get_image("ground_left.png");

                tile_width = ground_left.width();
                tile_height = ground_left.height();
            }
            TileType::Center => {
                let ground_centre = asset_manager.get_image("ground_centre.png");

                tile_width = ground_centre.width();
                tile_height = ground_centre.height();
            }
            TileType::Right => {
                let ground_right = asset_manager.get_image("ground_right.png");

                tile_width = ground_right.width();
                tile_height = ground_right.height();
            }
        }

        let body = physics.create_tile(na::Point2::new(pos_x, pos_y), tile_width, tile_height);

        Self {
            tile_type,
            body,

            width: tile_width as f32,
            height: tile_height as f32,
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
            TileType::Left => {
                graphics::draw(
                    ctx,
                    &ground_left,
                    DrawParam::default()
                        .dest(Point2::new(tile_position.x, tile_position.y))
                        .offset(Point2::new(0.5, 0.5)),
                )?;
            }

            TileType::Center => {
                graphics::draw(
                    ctx,
                    &ground_centre,
                    DrawParam::default()
                        .dest(Point2::new(tile_position.x, tile_position.y))
                        .offset(Point2::new(0.5, 0.5)),
                )?;
            }
            TileType::Right => {
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
