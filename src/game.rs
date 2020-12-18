use std::{io::Read, sync::Mutex};

use ggez::{
    event::KeyCode,
    graphics::{self, Color, DrawMode, DrawParam, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::{Font, Image, Mesh, TextFragment};

use crate::{HEIGHT, Screen, WIDTH, components::{bullet::Turbofish, enemy::Enemy, player::Player, tile::{Tile, TileType}}};

pub struct Game {
    ground: Vec<Tile>,
    enemies: Vec<Enemy>,
    player_bullets: Vec<Turbofish>,
    player: Player,

    ground_resources: Vec<Image>,
    enemy_resources: Vec<Image>,
    player_resources: Vec<Image>,
    bullet_resources: Vec<Image>,

    consolas: Font,

    ui_resources: Vec<Image>,

    camera: Camera,
}

impl Game {
    pub fn create(ctx: &mut Context) -> Mutex<Self> {
        let mut camera = Camera::new(WIDTH as u32, HEIGHT as u32, WIDTH, HEIGHT);
        let mut map = ggez::filesystem::open(ctx, "/maps/01.map").unwrap();

        let mut buffer = String::new();
        map.read_to_string(&mut buffer).unwrap();

        let mut ground = vec![];
        let mut enemies = vec![];
        let mut player = None;

        let mut draw_pos = 0.;

        let draw_inc = 64.;

        for id in buffer.chars() {
            match id {
                '[' => {
                    ground.push(Tile::new(draw_pos, TileType::LEFT));

                    draw_pos += draw_inc;
                }

                '-' => {
                    ground.push(Tile::new(draw_pos, TileType::CENTER));

                    draw_pos += draw_inc;
                }

                ']' => {
                    ground.push(Tile::new(draw_pos, TileType::RIGHT));

                    draw_pos += draw_inc;
                }

                '_' => {
                    draw_pos += draw_inc;
                }

                '8' => {
                    ground.push(Tile::new(draw_pos, TileType::CENTER));
                    enemies.push(Enemy::new(draw_pos));

                    draw_pos += draw_inc;
                }

                '4' => {
                    ground.push(Tile::new(draw_pos, TileType::CENTER));
                    player = Some(Player::new(draw_pos));

                    draw_pos += draw_inc;
                }

                _ => {}
            }
        }

        let player = player.expect("No player found!");

        camera.move_to(Vec2::new(player.pos_x, player.pos_y));

        Mutex::new(Self {
            ground,
            enemies,
            player,
            player_bullets: vec![],

            ground_resources: vec![
                Image::new(ctx, "/images/ground_left.png").unwrap(),
                Image::new(ctx, "/images/ground_centre.png").unwrap(),
                Image::new(ctx, "/images/ground_right.png").unwrap(),
            ],

            enemy_resources: vec![
                Image::new(ctx, "/images/gopher.png").unwrap(),
                Image::new(ctx, "/images/Some(gun).png").unwrap(),
            ],

            player_resources: vec![
                Image::new(ctx, "/images/Some(ferris).png").unwrap(),
                Image::new(ctx, "/images/Some(sniper).png").unwrap(),
            ],

            bullet_resources: vec![
                Image::new(ctx, "/images/Some(turbofish).png").unwrap(),
            ],

            ui_resources: vec![Image::new(ctx, "/images/Some(ammo).png").unwrap()],

            camera,

            consolas: graphics::Font::new(ctx, "/fonts/Consolas.ttf").unwrap(),
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Moon
        let moon = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(WIDTH - 40.0, 40.0),
            25.0,
            0.001,
            Color::from_rgb(255, 255, 255),
        )?;

        graphics::draw(ctx, &moon, DrawParam::default())?;

        // Ground
        for tile in &mut self.ground {
            tile.draw(ctx, &self.camera, &self.ground_resources)?;
        }

        // Enemies
        for enemy in &mut self.enemies {
            enemy.draw(ctx, &self.camera, &self.enemy_resources)?;
        }

        // Player
        self.player
            .draw(ctx, &self.camera, &self.player_resources)?;

        graphics::draw(
            ctx,
            &self.ui_resources[0],
            DrawParam::default().dest(Point2::new(10.0, 10.0)),
        )?;

        let ammo_frag = TextFragment {
            text: self.player.ammo.to_string(),
            font: Some(self.consolas),
            scale: Some(Scale::uniform(30.0)),
            color: {
                if self.player.ammo > 10 / 2 {
                    Some(Color::from_rgb(255, 255, 255))
                }

                else {
                    Some(Color::from_rgb(255, 80, 76))
                }
            },

            ..Default::default()
        };

        graphics::draw(
            ctx,
            &Text::new(ammo_frag),
            DrawParam::default().dest(Point2::new(30.0, 25.0)),
        )?;

        for fish in &mut self.player_bullets {
            fish.draw(ctx, &self.camera, &self.bullet_resources)?;
        }

        graphics::present(ctx)
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        let ferris_pos_x = self.player.pos_x;
        let mut ferris_is_falling_down: bool = true;

        for tile in &mut self.ground {
            let tile_start = tile.pos_x;
            let tile_end = tile.pos_x + 64.;

            if ferris_pos_x >= tile_start && ferris_pos_x <= tile_end {
                ferris_is_falling_down = false;

                break;
            }
        }

        self.player.update(ferris_is_falling_down);

        self.camera
            .move_to(Vec2::new(self.player.pos_x, self.player.pos_y));

        if self.player.pos_y < -800. {
            return Ok(Some(Screen::Dead));
        }

        for i in 0..self.enemies.len() {
            let go = &self.enemies[i];

            let go_start_x = go.pos_x;
            let go_end_x = go.pos_x + 100.;

            let mut done: bool = false;

            for fish in &mut self.player_bullets {
                if fish.pos_x >= go_start_x && fish.pos_x <= go_end_x {
                    self.enemies.remove(i);

                    done = true;
                }
            }

            if done {
                break;
            }
        }

        for i in 0..self.player_bullets.len() {
            let fish = &mut self.player_bullets[i];

            if fish.go_boom() {
                self.player_bullets.remove(i);

                break;
            }
        }

        Ok(None)
    }

    pub fn key_press(&mut self, keycode: KeyCode) -> Option<crate::Screen> {
        match keycode {
            KeyCode::Left => {
                self.player.pos_x -= 10.;
            }
            KeyCode::Right => {
                self.player.pos_x += 10.;
            }
            KeyCode::Space => {
                self.player.go_boom();
            },
            KeyCode::S => {
                if let Some(fish) = self.player.shoot() {
                    self.player_bullets.push(fish);
                }
            }
            _ => (),
        }

        None
    }
}
