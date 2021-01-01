use ggez::{
    graphics,
    mint::{Point2, Vector2},
    Context, GameResult,
};
use graphics::DrawParam;

use crate::{WIDTH, utils::{AssetManager, Position}};

pub struct Cloud {
    position: Position,

    scale: f32,
    speed: f32,
}

impl Cloud {
    pub fn new(pos_x: f32, pos_y: f32, scale: f32, speed: f32, asset_manager: &AssetManager) -> Self {
        let cloud = asset_manager.get_image("Some(cloud).png");
        let position = Position::new(pos_x, pos_y, cloud.width(), cloud.height());
        
        Self {
            position,
            scale,
            speed,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult<()> {
        let cloud = asset_manager.get_image("Some(cloud).png");

        graphics::draw(
            ctx,
            &cloud,
            DrawParam::default()
                .scale(Vector2 {
                    x: self.scale,
                    y: self.scale,
                })
                .dest(Point2 {
                    x: self.position.pos_start.x,
                    y: self.position.pos_start.y,
                }),
        )?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, asset_manager: &AssetManager) {
        let cloud = asset_manager.get_image("Some(cloud).png");
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        self.position.move_by("x+", delta_time * self.speed);

        if self.position.pos_start.x > WIDTH + 100. {
            self.position = Position::new(-100., self.position.pos_start.y, cloud.width(), cloud.height());
        }
    }
}
