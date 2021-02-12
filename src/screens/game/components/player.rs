use ggez::{event::KeyCode, graphics, input::keyboard, nalgebra::Point2, Context, GameResult};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::DrawParam;
use nphysics2d::object::DefaultBodyHandle;
use nphysics2d::{algebra::Velocity2, nalgebra as na};

use crate::{
    game::physics::{isometry_to_point, point_to_isometry, Physics},
    utils::AssetManager,
};

use super::bullet::{Grappling, PlayerWeapon, Turbofish, WeaponType};

pub enum Direction {
    Left,
    Right,
    None,
}

pub struct Player {
    pub ammo: f32,
    pub health: i32,

    direction: Direction,

    body: DefaultBodyHandle,
    pub weapons: Vec<PlayerWeapon>,
}

impl Player {
    const SHIFT_JUICE: f32 = 10.0;
    const JUMP_JUICE: f32 = 20.0;

    pub fn new(
        ctx: &mut Context,
        pos_x: f32,
        physics: &mut Physics,
        asset_manager: &AssetManager,
    ) -> Self {
        let (_, height) = graphics::drawable_size(ctx);

        let ferris = asset_manager.get_image("Some(ferris).png");

        let body = physics.create_player(
            na::Point2::new(pos_x, height / 2.0 - 155.),
            ferris.width(),
            ferris.height(),
        );

        let weapons = vec![];

        Self {
            ammo: 10.0,
            health: 100,

            direction: Direction::None,

            body,
            weapons,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        physics: &mut Physics,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let ferris = asset_manager.get_image("Some(ferris).png");
        let turbofish_sniper = asset_manager.get_image("Some(sniper).png");

        let player_position = self.position(physics);
        let ferris_position =
            camera.calculate_dest_point(Vec2::new(player_position.x, player_position.y));

        // Draw the player
        graphics::draw(
            ctx,
            &ferris,
            DrawParam::default()
                .dest(Point2::new(ferris_position.x, ferris_position.y))
                .offset(Point2::new(0.5, 0.5)),
        )?;

        graphics::draw(
            ctx,
            &turbofish_sniper,
            DrawParam::default()
                .dest(Point2::new(
                    ferris_position.x + 30.0,
                    ferris_position.y + 15.0,
                ))
                .offset(Point2::new(0.5, 0.5)),
        )?;

        // Draw the player weapon
        for weapon in &mut self.weapons {
            match weapon {
                PlayerWeapon::Turbofish(fish) => {
                    fish.draw(ctx, camera, physics, asset_manager)?;
                }
                PlayerWeapon::Grappling(grapple) => {
                    grapple.draw(ctx, camera, physics)?;
                }
            }
        }

        Ok(())
    }

    pub fn init(&mut self, physics: &mut Physics) {
        let player_body = physics.get_rigid_body_mut(self.body);
        let player_position = isometry_to_point(player_body.position());

        let updated_position =
            point_to_isometry(na::Point2::new(player_position.x, player_position.y - 40.0));

        player_body.set_position(updated_position);
    }

    pub fn update(&mut self, ctx: &mut Context, physics: &mut Physics) {
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.shift(physics, Direction::Left);
            self.set_direction(Direction::Left);
        } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.shift(physics, Direction::Right);
            self.set_direction(Direction::Right);
        }

        // We are not adding Space key pressed in an else if statement as we want to jump while we are also moving to a specific direction in the x axis.
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.go_boom(physics);
            self.set_direction(Direction::None);
        }

        // Same as the previous if statement. We want to shoot while moving and jumping around :)
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            // TODO: Move the shoot logic from game struct to this if statement
        }

        for i in 0..self.weapons.len() {
            let weapon = &mut self.weapons[i];

            match weapon {
                PlayerWeapon::Turbofish(fish) => {
                    if fish.update(physics) {
                        fish.destroy(physics);
                        self.weapons.remove(i);

                        break;
                    }
                }
                PlayerWeapon::Grappling(grapple) => {
                    if keyboard::is_key_pressed(ctx, KeyCode::S) {
                        grapple.update(physics);
                    } else {
                        self.weapons.remove(i);
                        break;
                    }
                }
            }
        }
    }

    pub fn shoot(
        &mut self,
        physics: &mut Physics,
        asset_manager: &AssetManager,
        gun: &WeaponType,
    ) -> Option<PlayerWeapon> {
        let player_position = self.position(physics);

        if self.ammo > 0.0 {
            match gun {
                WeaponType::Turbofish => Some(PlayerWeapon::Turbofish(Turbofish::new(
                    player_position.x + 140.0,
                    player_position.y,
                    physics,
                    asset_manager,
                ))),

                WeaponType::Grappling => {
                    let gun = Grappling::new(
                        player_position.x + 140.0,
                        player_position.y,
                        physics,
                        self.handle(),
                    );

                    if let Some(grapple) = gun {
                        Some(PlayerWeapon::Grappling(grapple))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn position(&mut self, physics: &mut Physics) -> na::Point2<f32> {
        let player_body = physics.get_rigid_body_mut(self.body);
        let player_position = isometry_to_point(player_body.position());

        player_position
    }

    pub fn go_boom(&mut self, physics: &mut Physics) {
        let player_body = physics.get_rigid_body_mut(self.body);
        let player_velocity = player_body.velocity();

        let new_velocity = Velocity2::new(
            na::Vector2::new(
                player_velocity.linear.x,
                player_velocity.linear.y - Self::JUMP_JUICE,
            ),
            player_velocity.angular,
        );

        player_body.set_velocity(new_velocity);
    }

    fn shift(&mut self, physics: &mut Physics, direction: Direction) {
        let player_body = physics.get_rigid_body_mut(self.body);
        let player_velocity = player_body.velocity();

        match direction {
            Direction::Left => {
                let new_velocity = Velocity2::new(
                    na::Vector2::new(
                        player_velocity.linear.x - Self::SHIFT_JUICE,
                        player_velocity.linear.y,
                    ),
                    player_velocity.angular,
                );

                player_body.set_velocity(new_velocity);
            }
            Direction::Right => {
                let new_velocity = Velocity2::new(
                    na::Vector2::new(
                        player_velocity.linear.x + Self::SHIFT_JUICE,
                        player_velocity.linear.y,
                    ),
                    player_velocity.angular,
                );

                player_body.set_velocity(new_velocity);
            }
            Direction::None => {
                panic!("Direction::None direction was passed in the Player::move_x() function where None value of the Direction enum was not expected. Panic!");
            }
        }
    }

    pub fn handle(&self) -> DefaultBodyHandle {
        self.body
    }
}
