use ggez::{graphics::Image, Context, GameResult};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
};

use crate::{HEIGHT, utils::lerp};

use super::bullet::Turbofish;

pub enum Direction {
    Left,
    Right,
    None
}

pub struct Player {
    pub pos_x: f32,
    pub pos_y: f32,
    pub ammo: i32,

    gravity: f32,
    velocity: f32,
    going_boom: bool,
    lerp_to: Option<f32>,
    direction: Direction
}

impl Player {
    pub fn new(pos_x: f32) -> Self {
        Self {
            pos_x,
            ammo: 10,
            pos_y: 0.,
            gravity: 0.1,
            velocity: 0.,
            going_boom: false,
            lerp_to: None,
            direction: Direction::None
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        resources: &Vec<Image>,
    ) -> GameResult<()> {
        const HEIGHT2: f32 = HEIGHT / 2.;

        &resources[0].draw_camera(
            &camera,
            ctx,
            Vec2::new(self.pos_x, (-HEIGHT2 + 155.) + self.pos_y),
            0.0,
        );

        &resources[1].draw_camera(
            &camera,
            ctx,
            Vec2::new(self.pos_x + 30., (-HEIGHT2 + 120.) + self.pos_y),
            0.0,
        );

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
            }

            else {
                self.pos_x = lerp(self.pos_x, l, 0.4);
            }
        }
        
        if self.going_boom {
            self.pos_y -= self.velocity;

            if self.pos_y < 0. {
                self.going_boom = false;

                self.pos_y = 0.;
                self.velocity = 0.;
            }
        }

        if self.pos_y > 0. || gonna_boom {
            self.velocity += self.gravity;
            self.pos_y -= self.velocity;
        }
    }

    pub fn shoot(&mut self) -> Option<Turbofish> {
        const HEIGHT2: f32 = HEIGHT / 2.;

        if self.ammo != 0 {
            self.ammo -= 1;

            return Some(Turbofish::new(
                self.pos_x + 220.,
                (-HEIGHT2 + 106.) + self.pos_y,
            ));
        } else {
            None
        }
    }
}
