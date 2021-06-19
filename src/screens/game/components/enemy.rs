use ggez::{
    audio::SoundSource,
    graphics::{self, DrawParam},
    nalgebra::Point2,
    Context, GameResult,
};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};

use nphysics2d::{nalgebra as na, object::DefaultBodyHandle};

use crate::{
    game::physics::{isometry_to_point, Physics},
    play,
    utils::{AssetManager, ParticleSystem},
};

use super::{bullet::PlayerWeapon, player::Player};

pub struct Enemy {
    body: DefaultBodyHandle,
}

impl Enemy {
    pub fn new(
        ctx: &mut Context,
        pos_x: f32,
        physics: &mut Physics,
        asset_manager: &AssetManager,
    ) -> Self {
        let (_, height) = graphics::drawable_size(ctx);

        let gopher = asset_manager.get_image("gopher.png");

        let body = physics.create_enemy(
            na::Point2::new(pos_x, height / 2.0 - 155.0),
            gopher.width(),
            gopher.height(),
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
        let gopher = asset_manager.get_image("gopher.png");
        let gun = asset_manager.get_image("Some(gun).png");

        let enemy_position = self.position(physics);
        let gopher_position =
            camera.calculate_dest_point(Vec2::new(enemy_position.x, enemy_position.y));

        graphics::draw(
            ctx,
            &gopher,
            DrawParam::default()
                .dest(Point2::new(gopher_position.x, gopher_position.y))
                .offset(Point2::new(0.5, 0.5)),
        )?;

        graphics::draw(
            ctx,
            &gun,
            DrawParam::default()
                .dest(Point2::new(
                    gopher_position.x - 50.0,
                    gopher_position.y + 10.0,
                ))
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
        let position = self.position(physics);

        let gopher = asset_manager.get_image("gopher.png");
        let explode_sound = asset_manager.get_sound("Some(explode).mp3");

        for i in 0..player.weapons.len() {
            match &mut player.weapons[i] {
                PlayerWeapon::Turbofish(fish) => {
                    if fish.is_touching(physics, self.handle()) {
                        particles.push(ParticleSystem::new(
                            physics,
                            50,
                            na::Point2::new(
                                position.x - (gopher.width() / 2) as f32,
                                position.y - (gopher.height() / 2) as f32,
                            ),
                            na::Point2::new(
                                position.x + (gopher.width() / 2) as f32,
                                position.y + (gopher.height() / 2) as f32,
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

        // Can the enemy see the player?
        if physics.distance(self.handle(), player.handle()) < 300.0 {
            // TODO: The enemy shoots the player as soon as it see's the player.
        }

        false
    }

    pub fn position(&self, physics: &mut Physics) -> na::Point2<f32> {
        let enemy_body = physics.get_rigid_body_mut(self.body);
        let enemy_position = isometry_to_point(enemy_body.position());

        enemy_position
    }

    pub fn handle(&mut self) -> DefaultBodyHandle {
        self.body
    }

    pub fn destroy(&mut self, physics: &mut Physics) {
        physics.destroy_body(self.body);
    }
}
