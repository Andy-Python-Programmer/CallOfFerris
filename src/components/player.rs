use ggez::{event::KeyCode, graphics, input::keyboard, nalgebra::Point2, Context, GameResult};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::DrawParam;
use nphysics2d::object::DefaultBodyHandle;
use nphysics2d::{algebra::Velocity2, nalgebra as na};

use crate::{
    physics::{isometry_to_point, point_to_isometry, Physics},
    utils::AssetManager,
    HEIGHT,
};

const HEIGHT2: f32 = HEIGHT / 2.;

use super::{
    bullet::{Grappling, PlayerWeapon, Turbofish},
    enemy::Enemy,
};

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
}

impl Player {
    pub fn new(pos_x: f32, physics: &mut Physics, asset_manager: &AssetManager) -> Self {
        let ferris = asset_manager.get_image("Some(ferris).png");

        let body = physics.create_player(
            na::Point2::new(pos_x, HEIGHT2 - 155.),
            ferris.width(),
            ferris.height(),
        );

        Self {
            ammo: 10.0,
            health: 100,

            direction: Direction::None,

            body,
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
            self.move_x(physics, Direction::Left);
            self.set_direction(Direction::Left);
        } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.move_x(physics, Direction::Right);
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
    }

    pub fn shoot(
        &mut self,
        physics: &mut Physics,
        asset_manager: &AssetManager,
        gun: &str,
        enemies: &Vec<Enemy>,
    ) -> Option<PlayerWeapon> {
        let player_position = self.position(physics);

        if !(self.ammo <= 0.0) {
            match gun {
                "Turbofish Gun" => Some(PlayerWeapon::Turbofish(Turbofish::new(
                    player_position.x + 140.0,
                    player_position.y,
                    physics,
                    asset_manager,
                ))),

                "Grappling Gun" => {
                    let gun = Grappling::new(
                        player_position.x + 220.0,
                        player_position.y - 49.0,
                        asset_manager,
                        enemies,
                    );

                    if let Some(grapple) = gun {
                        Some(PlayerWeapon::Grappling(grapple))
                    } else {
                        None
                    }
                }

                _ => {
                    panic!()
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
            na::Vector2::new(player_velocity.linear.x, player_velocity.linear.y - 20.0),
            player_velocity.angular,
        );

        player_body.set_velocity(new_velocity);
    }

    fn move_x(&mut self, physics: &mut Physics, direction: Direction) {
        let player_body = physics.get_rigid_body_mut(self.body);
        let player_velocity = player_body.velocity();

        match direction {
            Direction::Left => {
                let new_velocity = Velocity2::new(
                    na::Vector2::new(player_velocity.linear.x - 10.0, player_velocity.linear.y),
                    player_velocity.angular,
                );

                player_body.set_velocity(new_velocity);
            }
            Direction::Right => {
                let new_velocity = Velocity2::new(
                    na::Vector2::new(player_velocity.linear.x + 10.0, player_velocity.linear.y),
                    player_velocity.angular,
                );

                player_body.set_velocity(new_velocity);
            }
            Direction::None => {
                panic!("Direction::None direction was passed in the Player::move_x() function where None value of the Direction enum was not expected. Panic!!");
            }
        }
    }
}
