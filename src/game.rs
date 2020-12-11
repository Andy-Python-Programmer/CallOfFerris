use std::sync::Mutex;

use ggez::{
    audio::SoundSource, audio::Source, event::KeyCode, graphics, graphics::DrawParam, timer,
    Context, GameResult,
};
use rand::Rng;

use crate::HEIGHT;
use crate::WIDTH;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum AppleType {
    Referenced,
    Dereferenced,
}

pub struct Game {
    pub ferris_death_audio: Source,
    pub ferris_pacman: graphics::Image,

    pub ferris_pacman_collection: Vec<graphics::Image>,

    pub ferris_pos: (f32, f32), // X, Y
    pub ferris_direction: Direction,
    pub points: i64,
    pub hp: i32,

    pub food: Vec<(AppleType, graphics::Image, f32, f32)>, // Type of the apple, Image of the apple, X pos of the apple, Y pos of the apple
}

impl Game {
    pub fn create(ctx: &mut Context) -> Mutex<Self> {
        let mut food_vector = Vec::new();
        let mut rng = rand::thread_rng();

        for _i in 0..10 {
            let apple_type_num = rng.gen_range(0, 10);

            let pos_x: i64;
            let pos_y: i64;

            loop {
                let gen_x = rng.gen_range(0, WIDTH as i32);

                if gen_x % 20 == 0 {
                    pos_x = gen_x.into();

                    break;
                }
            }

            loop {
                let gen_y = rng.gen_range(0, HEIGHT as i32);

                if gen_y % 20 == 0 {
                    pos_y = gen_y.into();

                    break;
                }
            }

            if apple_type_num % 2 == 0 {
                food_vector.push((
                    AppleType::Referenced,
                    graphics::Image::new(ctx, "/images/apple_reference.png").unwrap(),
                    pos_x as f32,
                    pos_y as f32,
                ));
            } else {
                food_vector.push((
                    AppleType::Dereferenced,
                    graphics::Image::new(ctx, "/images/apple_dereference.png").unwrap(),
                    pos_x as f32,
                    pos_y as f32,
                ));
            }
        }

        Mutex::new(Self {
            ferris_death_audio: Source::new(ctx, "/audio/dead.mp3").unwrap(),
            ferris_pacman: graphics::Image::new(ctx, "/images/ferris_pacman_1.png").unwrap(),
            ferris_pacman_collection: vec![
                graphics::Image::new(ctx, "/images/ferris_pacman_1.png").unwrap(),
                graphics::Image::new(ctx, "/images/ferris_pacman_2.png").unwrap(),
            ],

            ferris_pos: (20.0, 20.0),
            ferris_direction: Direction::Right,

            points: 0,
            hp: 40,

            food: food_vector,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for food in self.food.iter() {
            graphics::draw(ctx, &food.1, (ggez::nalgebra::Point2::new(food.2, food.3),)).unwrap();
        }

        let hp_pos_y;
        let hp_pos_x;

        graphics::draw(
            ctx,
            &self.ferris_pacman,
            DrawParam::default()
                .dest(ggez::nalgebra::Point2::new(
                    self.ferris_pos.0,
                    self.ferris_pos.1,
                ))
                .rotation(if self.ferris_direction == Direction::Up {
                    hp_pos_y = self.ferris_pos.1 + 10.0;
                    hp_pos_x = self.ferris_pos.0 - 20.0;

                    80.0
                } else if self.ferris_direction == Direction::Down {
                    hp_pos_y = self.ferris_pos.1 - 50.0;
                    hp_pos_x = self.ferris_pos.0 - 80.0;

                    -80.0
                } else if self.ferris_direction == Direction::Left {
                    hp_pos_y = self.ferris_pos.1 - 100.0;
                    hp_pos_x = self.ferris_pos.0 - 80.0;

                    160.0
                } else {
                    hp_pos_y = self.ferris_pos.1 - 50.0;
                    hp_pos_x = self.ferris_pos.0 - 20.0;

                    0.0
                }),
        )?;

        let hp_rect_full = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(hp_pos_x, hp_pos_y, 100.0, 20.0),
            [1.0, 0.0, 0.0, 1.0].into(),
        )?;

        let hp_rect_cur = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(hp_pos_x, hp_pos_y, self.hp as f32, 20.0),
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;

        graphics::draw(ctx, &hp_rect_full, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &hp_rect_cur, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        graphics::present(ctx)
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        if timer::ticks(ctx) % 6 == 0 {
            if self.ferris_pacman == self.ferris_pacman_collection[0] {
                self.ferris_pacman = self.ferris_pacman_collection[1].to_owned();
            } else {
                self.ferris_pacman = self.ferris_pacman_collection[0].to_owned();
            }

            for i in 0..self.food.len() - 1 {
                let apple = &self.food[i];

                if apple.2 == self.ferris_pos.0 && apple.3 == self.ferris_pos.1 {
                    if apple.0 == AppleType::Referenced {
                        self.points += 1;

                        if self.hp != 100 {
                            self.hp += 20;
                        }

                        self.food.remove(i);
                    } else {
                        self.points -= 1;

                        if self.hp - 20 != 0 {
                            self.hp -= 20;
                        } else {
                            self.hp = 0;
                            self.ferris_death_audio.play()?;

                            return Ok(Some(crate::Screen::Dead));
                        }

                        self.food.remove(i);
                    }
                }
            }

            if self.ferris_direction == Direction::Left {
                self.ferris_pos.0 -= 20.0;
            } else if self.ferris_direction == Direction::Right {
                self.ferris_pos.0 += 20.0;
            } else if self.ferris_direction == Direction::Up {
                self.ferris_pos.1 -= 20.0;
            } else if self.ferris_direction == Direction::Down {
                self.ferris_pos.1 += 20.0;
            }
        }

        if self.ferris_pos.0 > WIDTH {
            self.ferris_pos.0 = 0.0;
        } else if self.ferris_pos.0 < 0.0 {
            self.ferris_pos.0 = WIDTH;
        } else if self.ferris_pos.1 > HEIGHT {
            self.ferris_pos.1 = 0.0;
        } else if self.ferris_pos.1 < 0.0 {
            self.ferris_pos.1 = HEIGHT;
        }

        if self.hp == 0 {
            self.ferris_death_audio.play()?;

            return Ok(Some(crate::Screen::Dead));
        }

        Ok(None)
    }

    pub fn key_press(&mut self, keycode: KeyCode) -> Option<crate::Screen> {
        if keycode == KeyCode::Up {
            self.ferris_direction = Direction::Up;
        } else if keycode == KeyCode::Down {
            self.ferris_direction = Direction::Down;
        } else if keycode == KeyCode::Left {
            self.ferris_direction = Direction::Left;
        } else if keycode == KeyCode::Right {
            self.ferris_direction = Direction::Right;
        }

        None
    }
}
