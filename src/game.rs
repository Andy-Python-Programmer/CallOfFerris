use std::{io::Read, sync::Mutex};

use ggez::{Context, GameResult, audio::{SoundSource, Source}, event::KeyCode, graphics::{self, Color, DrawMode, DrawParam, Scale, Text}, mint, nalgebra::Point2, timer};
use ggez_goodies::{camera::{Camera, CameraDraw}, nalgebra_glm::Vec2, particle::{EmissionShape, ParticleSystem, ParticleSystemBuilder, Transition}};
use graphics::{Font, Image, Mesh, TextFragment};
use rand::Rng;

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
    audio_resources: Vec<Source>,

    camera: Camera,
    elapsed_shake: Option<(f32, Vec2)>,
    tics: Option<i32>,
    particles: Vec<(ParticleSystem, f32, f32, i32)>
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
            audio_resources: vec![
                Source::new(ctx, "/audio/Some(turbofish_shoot).mp3").unwrap(),
                Source::new(ctx, "/audio/Some(explode).mp3").unwrap()
            ],

            camera,

            consolas: graphics::Font::new(ctx, "/fonts/Consolas.ttf").unwrap(),
            elapsed_shake: None,
            tics: None,
            particles: vec![]
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

        for sys in &mut self.particles {
            &sys.0.draw_camera(
                &self.camera,
                ctx,
                Vec2::new(sys.1, sys.2),
                0.
            );
        }

        graphics::present(ctx)
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        if let Some(t) = self.tics {
            if timer::ticks(ctx) % t as usize == 0 {
                return self.inner_update(ctx);
            }
        }

        else {
            return self.inner_update(ctx);
        }

        Ok(None)
    }

    pub fn inner_update(&mut self, ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        let ferris_pos_x = self.player.pos_x;
        let ferris_pos_y = self.player.pos_y;

        let mut ferris_is_falling_down = true;

        for tile in &mut self.ground {
            let tile_start = tile.pos_x;
            let tile_end = tile.pos_x + 64.;

            if ferris_pos_x >= tile_start && ferris_pos_x <= tile_end && ferris_pos_y + (-HEIGHT / 2.0) - 64. >= (-HEIGHT / 2.0) - 64. {
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

            for fish in &self.player_bullets {
                if fish.pos_x >= go_start_x && fish.pos_x <= go_end_x {
                    const HEIGHT2: f32 = HEIGHT / 2.;

                    self.particles.push(
                        (ParticleSystemBuilder::new(ctx)
                            .count(100)
                            .emission_rate(100.0)
                            .start_max_age(5.0)
                            .start_size_range(2.0, 15.0)
                            .start_color_range(
                                graphics::Color::from((0, 0, 0)),
                                graphics::Color::from((255, 255, 255)),
                            )
                            .delta_color(Transition::range(
                                ggez::graphics::Color::from((255, 0, 0)),
                                ggez::graphics::Color::from((255, 255, 0)),
                            ))
                            .emission_shape(EmissionShape::Circle(mint::Point2 { x: 0.0, y: 0.0 }, 100.0))
                            .build(), go_start_x, -HEIGHT2 + 70., 0)
                    );

                    self.audio_resources[1].play().expect("Cannot play Some(explode).mp3");

                    self.enemies.remove(i);

                    done = true;
                }
            }

            if done {
                let cam_loc = self.camera.location();
                let org_pos = cam_loc.data.as_slice();
                
                self.elapsed_shake = Some((0., Vec2::new(org_pos[0], org_pos[1])));
                self.camera_shakeke();

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

        if let Some(s) = self.elapsed_shake {
            if s.0 < 1. {
                self.camera_shakeke();
            }

            else {
                self.camera.move_to(s.1);
                self.elapsed_shake = None;
            }
        }

        for sys in &mut self.particles {
            sys.0.update(0.5);
            sys.3 += 1;
        }

        for i in 0..self.particles.len() {
            let sys = &self.particles[i];

            if sys.3 > 6 {
                self.particles.remove(i);
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
                    self.audio_resources[0].play().expect("Cannot play Some(turbofish_shoot).mp3");
                    
                    self.player_bullets.push(fish);
                }
            },
            KeyCode::F3 => {
                self.tics = Some(6);
            },
            _ => (),
        }

        None
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::F3 => {
                self.tics = None;
            },

            _ => ()
        }
    }

    pub fn camera_shakeke(&mut self) {
        let mut rng = rand::thread_rng();
    
        let magnitude = 3.;
        let elapsed = self.elapsed_shake.unwrap();
    
        let x = rng.gen_range(-1.0, 1.0) * magnitude;
        let y = rng.gen_range(-1.0, 1.0) * magnitude;

        self.camera.move_by(Vec2::new(x, y));
    
        self.elapsed_shake = Some((elapsed.0 + 0.1, elapsed.1));
    }
}
