use ggez::{audio::SoundSource, graphics, nalgebra::Point2, Context, GameResult};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::DrawParam;

use crate::{
    game::physics::{isometry_to_point, Physics},
    play,
    utils::{AssetManager, ParticleSystem},
    HEIGHT,
};

const HEIGHT2: f32 = HEIGHT / 2.;

use nphysics2d::{nalgebra as na, object::DefaultBodyHandle};

use super::{bullet::PlayerWeapon, player::Player};

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

    pub fn update(
        &mut self,
        physics: &mut Physics,
        asset_manager: &AssetManager,
        particles: &mut Vec<ParticleSystem>,
        player: &mut Player,
    ) -> bool {
        let barrel = asset_manager.get_image("Some(barrel).png");

        let position = self.position(physics);

        for i in 0..player.weapons.len() {
            match &mut player.weapons[i] {
                PlayerWeapon::Turbofish(fish) => {
                    if fish.is_touching(physics, self.handle()) {
                        let explode_sound = asset_manager.get_sound("Some(explode).mp3");

                        // FIXME
                        particles.push(ParticleSystem::new(
                            physics,
                            100,
                            na::Point2::new(
                                position.x - (barrel.width() / 2) as f32,
                                position.y - (barrel.height() / 2) as f32,
                            ),
                            na::Point2::new(
                                position.x + (barrel.width() / 2) as f32,
                                position.y + (barrel.height() / 2) as f32,
                            ),
                        ));

                        play!(explode_sound);

                        // Remove the enemy from the world
                        self.destroy(physics);

                        // Remove the weapon from the world
                        fish.destroy(physics);
                        player.weapons.remove(i);

                        return true;
                    }
                }
                PlayerWeapon::Grappling(_) => {}
            }
        }

        false
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
