use ggez::{Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::{
    utils::{lerp, AssetManager},
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
    pub pos_x: f32,
    pub pos_y: f32,

    pub ammo: f32,
    pub health: i32,

    gravity: f32,
    velocity: f32,
    pub going_boom: bool,
    lerp_to: Option<f32>,
    direction: Direction,
}

impl Player {
    pub fn new(pos_x: f32) -> Self {
        Self {
            pos_x,
            ammo: 10.,
            pos_y: 0.,
            gravity: 0.1,
            velocity: 0.,
            going_boom: false,
            lerp_to: None,
            direction: Direction::None,
            health: 100,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        let ferris = asset_manager.get_image("Some(ferris).png");
        let turbofish_sniper = asset_manager.get_image("Some(sniper).png");

        ferris.draw_camera(
            &camera,
            ctx,
            Vec2::new(self.pos_x, (-HEIGHT2 + 155.) + self.pos_y),
            0.0,
        )?;

        turbofish_sniper.draw_camera(
            &camera,
            ctx,
            Vec2::new(self.pos_x + 30., (-HEIGHT2 + 120.) + self.pos_y),
            0.0,
        )?;

        Ok(())
    }

    pub fn go_boom(&mut self) {
        self.velocity -= 2.5;
        self.going_boom = true;

        match self.direction {
            Direction::Left => {
                self.lerp_to = Some(self.pos_x - 300.);
            }
            Direction::Right => {
                self.lerp_to = Some(self.pos_x + 300.);
            }
            Direction::None => {
                self.lerp_to = None;
            }
        }
    }

    pub fn update(&mut self, gonna_boom: bool) {
        if let Some(l) = self.lerp_to {
            if self.pos_x as i32 == l as i32 {
                self.lerp_to = None;
            } else {
                self.pos_x = lerp(self.pos_x, l, 0.5);
            }
        }

        if self.going_boom {
            self.pos_y -= self.velocity;

            if self.pos_y < 0. && !gonna_boom {
                self.going_boom = false;

                self.pos_y = 0.;
                self.velocity = 0.;
            }
        }

        if self.pos_y > 0. || gonna_boom {
            self.velocity += self.gravity;
            self.pos_y -= self.velocity;
        }

        if self.pos_y == 0.0 && self.velocity != 0.0 {
            self.velocity = 0.0;
        }
    }

    pub fn shoot(
        &mut self,
        asset_manager: &AssetManager,
        gun: &str,
        enemies: &Vec<Enemy>,
    ) -> Option<PlayerWeapon> {
        if self.ammo as i32 != 0 {
            match gun {
                "Turbofish Gun" => Some(PlayerWeapon::Turbofish(Turbofish::new(
                    self.pos_x + 220.,
                    (-HEIGHT2 + 106.) + self.pos_y,
                    asset_manager,
                ))),

                "Grappling Gun" => {
                    let gun = Grappling::new(
                        self.pos_x + 150.,
                        (-HEIGHT2 + 106.) + self.pos_y,
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

    pub fn move_x(&mut self, x: f32) {
        self.lerp_to = Some(lerp(self.pos_x, x, 2.5));
    }
}
