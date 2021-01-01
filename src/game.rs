use std::{collections::HashMap, io::Read, process::exit, rc::Rc, sync::Mutex};

use ggez::{
    audio::SoundSource,
    event::KeyCode,
    graphics::{self, Color, DrawParam, Drawable, Shader, Text},
    mint,
    nalgebra::Point2,
    timer, Context, GameResult,
};
use ggez_goodies::{
    camera::{Camera, CameraDraw},
    nalgebra_glm::Vec2,
    particle::{EmissionShape, ParticleSystem, ParticleSystemBuilder, Transition},
};
use graphics::{Font, GlBackendSpec, Scale, ShaderGeneric, TextFragment};
use mint::Vector2;
use rand::Rng;

use crate::{
    components::{
        barrel::Barrel,
        bullet::Turbofish,
        cloud::Cloud,
        enemy::Enemy,
        player::{Direction, Player},
        tile::Tile,
    },
    map::Map,
    utils::{lerp, remap, AssetManager},
    Screen, HEIGHT, WIDTH,
};

use gfx::*;

gfx_defines! {
    constant Dim {
        rate: f32 = "u_Rate",
    }
}

const HEIGHT2: f32 = HEIGHT / 2.;

pub struct Game {
    ground: Vec<Tile>,
    clouds: Vec<Cloud>,
    enemies: Vec<Enemy>,
    barrels: Vec<Barrel>,

    player_bullets: Vec<Turbofish>,
    player: Player,

    asset_manager: Rc<AssetManager>,

    consolas: Font,

    camera: Camera,

    elapsed_shake: Option<(f32, Vec2, f32)>,
    tics: Option<i32>,
    particles: Vec<(ParticleSystem, f32, f32, i32)>,
    ui_lerp: HashMap<String, f32>,

    dim_shader: ShaderGeneric<GlBackendSpec, Dim>,
    dim_constant: Dim,

    end: Option<String>,

    draw_end_text: (bool, Option<usize>, bool, bool), // Thread Sleeped?, Current Iters, Done?, Win?
    can_die: bool,
    total_enemies: i32,
}

impl Game {
    pub fn create(ctx: &mut Context, asset_manager: Rc<AssetManager>) -> Mutex<Self> {
        let mut camera = Camera::new(WIDTH as u32, HEIGHT as u32, WIDTH, HEIGHT);
        let mut map = ggez::filesystem::open(ctx, "/maps/01.map").unwrap();

        let mut rng = rand::thread_rng();

        let mut buffer = String::new();
        map.read_to_string(&mut buffer).unwrap();

        let mut map_1 = Map::new();

        map_1.parse(buffer, &asset_manager);

        let ground = map_1.ground;
        let enemies = map_1.enemies;
        let barrels = map_1.barrels;
        let mut clouds = vec![];

        let player = map_1.player;

        let dim_constant = Dim { rate: 1.0 };

        let dim_shader = Shader::new(
            ctx,
            "/shaders/dim.basic.glslf",
            "/shaders/dim.glslf",
            dim_constant,
            "Dim",
            None,
        )
        .unwrap();

        let mut player = player.expect("No player found!");
        let mut ui_lerp = HashMap::new();

        ui_lerp.insert(String::from("ammo"), player.ammo as f32);
        ui_lerp.insert(String::from("health"), player.health as f32);

        player.pos_y += 40.;
        player.going_boom = true;

        camera.move_to(Vec2::new(player.pos_x, player.pos_y));

        for _ in 0..rng.gen_range(5, 7) {
            clouds.push(Cloud::new(
                rng.gen_range(0., WIDTH),
                rng.gen_range(10., 40.),
                rng.gen_range(0.1, 0.3),
                rng.gen_range(10., 35.),
                &asset_manager,
            ));
        }

        Mutex::new(Self {
            ground,
            clouds,
            enemies,
            player,
            barrels,

            asset_manager,

            player_bullets: vec![],

            camera,

            consolas: graphics::Font::new(ctx, "/fonts/Consolas.ttf").unwrap(),

            elapsed_shake: None,
            tics: None,
            particles: vec![],
            ui_lerp,

            dim_shader,
            dim_constant,
            draw_end_text: (false, None, false, false),
            end: map_1.end,
            can_die: true,
            total_enemies: map_1.total_enemies,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<Option<Screen>> {
        if let Some(_t) = self.tics {
            {
                let _lock = graphics::use_shader(ctx, &self.dim_shader);

                self.inner_draw(ctx)?;
            }

            if self.draw_end_text.0 && self.draw_end_text.3 {
                let mut draw_pos = 0.;

                // You Win
                let end_frag = &Text::new(
                    TextFragment::new("You Win!")
                        .font(self.consolas)
                        .scale(Scale::uniform(50.)),
                );

                let end_dimensions = end_frag.dimensions(ctx);

                graphics::draw(
                    ctx,
                    end_frag,
                    DrawParam::default().dest(Point2::new(
                        (WIDTH / 2.0) - (end_dimensions.0 / 2) as f32,
                        50.0,
                    )),
                )?;

                // End quote
                for line in self.end.as_ref().unwrap().split("\\n").collect::<Vec<_>>() {
                    let end_frag = &Text::new(TextFragment::new(line).font(self.consolas));

                    let end_dimensions = end_frag.dimensions(ctx);

                    graphics::draw(
                        ctx,
                        end_frag,
                        DrawParam::default().dest(Point2::new(
                            (WIDTH / 2.0) - (end_dimensions.0 / 2) as f32,
                            HEIGHT / 2. + draw_pos,
                        )),
                    )?;

                    draw_pos += 20.0;
                }

                // Press & to go to menu screen
                let menu_rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        (WIDTH / 2.) + 20.,
                        (HEIGHT / 2.) + (draw_pos * 2.),
                        220.0,
                        40.0,
                    ),
                    [36.0 / 255.0, 36.0 / 255.0, 36.0 / 255.0, 0.9].into(),
                )?;

                let menu_rect_dim = menu_rect.dimensions(ctx).unwrap();

                let menu_frag_to =
                    &Text::new(TextFragment::new("Press & go to the").font(self.consolas));

                let menu_screen = &Text::new(
                    TextFragment::new("MENU SCREEN")
                        .font(self.consolas)
                        .scale(Scale::uniform(20.0)),
                );

                graphics::draw(ctx, &menu_rect, DrawParam::default())?;
                graphics::draw(
                    ctx,
                    menu_frag_to,
                    DrawParam::default().dest(Point2::new(
                        (WIDTH / 2.) + 20.,
                        ((HEIGHT / 2.) + (draw_pos * 2.)) - 20.0,
                    )),
                )?;

                graphics::draw(
                    ctx,
                    menu_screen,
                    DrawParam::default().dest(Point2::new(
                        (WIDTH / 2.) + 70.,
                        ((HEIGHT / 2.) + (draw_pos * 2.)) + 12.0,
                    )),
                )?;

                // Press * to quit
                let quit_rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        ((WIDTH / 2.) - menu_rect_dim.w) - 20.0,
                        (HEIGHT / 2.) + (draw_pos * 2.),
                        220.0,
                        40.0,
                    ),
                    [36.0 / 255.0, 36.0 / 255.0, 36.0 / 255.0, 0.9].into(),
                )?;

                let quit_frag_to = &Text::new(TextFragment::new("Press * to").font(self.consolas));

                let press_quit = &Text::new(
                    TextFragment::new("QUIT")
                        .font(self.consolas)
                        .scale(Scale::uniform(20.)),
                );

                graphics::draw(ctx, &quit_rect, DrawParam::default())?;
                graphics::draw(
                    ctx,
                    quit_frag_to,
                    DrawParam::default().dest(Point2::new(
                        ((WIDTH / 2.) - menu_rect_dim.w) - 20.,
                        ((HEIGHT / 2.) + (draw_pos * 2.)) - 20.,
                    )),
                )?;

                graphics::draw(
                    ctx,
                    press_quit,
                    DrawParam::default().dest(Point2::new(
                        (((WIDTH / 2.) - menu_rect_dim.w) - 20.) + 90.,
                        (((HEIGHT / 2.) + (draw_pos * 2.)) - 20.) + 30.,
                    )),
                )?;
            }
        } else {
            self.inner_draw(ctx)?;
        }

        graphics::present(ctx)?;

        Ok(None)
    }

    fn inner_draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Clouds
        for cloud in &mut self.clouds {
            cloud.draw(ctx, &self.asset_manager)?;
        }

        // Ground
        for tile in &mut self.ground {
            tile.draw(ctx, &self.camera, &self.asset_manager)?;
        }

        // Enemies
        for enemy in &mut self.enemies {
            enemy.draw(ctx, &self.camera, &self.asset_manager)?;
        }

        // Barrel
        for boom in &mut self.barrels {
            boom.draw(ctx, &self.camera, &self.asset_manager)?;
        }

        // Player
        self.player.draw(ctx, &self.camera, &self.asset_manager)?;

        // Player Bullets
        for fish in &mut self.player_bullets {
            fish.draw(ctx, &self.camera, &self.asset_manager)?;
        }

        // Particles
        for sys in &mut self.particles {
            &sys.0
                .draw_camera(&self.camera, ctx, Vec2::new(sys.1, sys.2), 0.);
        }

        // User Profile, etc..
        self.draw_ui(ctx)?;

        Ok(())
    }

    fn draw_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let profile = self.asset_manager.get_image("Some(profile).png");
        let fish = self.asset_manager.get_image("Some(fish).png");

        graphics::draw(
            ctx,
            &profile,
            DrawParam::default()
                .dest(Point2::new(10.0, 10.0))
                .scale(Vector2 { x: 0.5, y: 0.5 }),
        )?;

        let ammo_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                ((profile.width() / 2) + 10) as f32,
                (profile.height() / 3) as f32,
                150.,
                15.,
            ),
            Color::from_rgb(54, 50, 49),
        )?;

        let hp_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                ((profile.width() / 2) + 10) as f32,
                (profile.height() / 5) as f32,
                150.,
                15.,
            ),
            Color::from_rgb(54, 50, 49),
        )?;

        let cur_ammo_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                ((profile.width() / 2) + 10) as f32,
                (profile.height() / 3) as f32,
                remap(self.player.ammo as f32, 0., 10., 0., 150.),
                15.,
            ),
            Color::from_rgb(21, 156, 228),
        )?;

        let cur_hp_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                ((profile.width() / 2) + 10) as f32,
                (profile.height() / 5) as f32,
                remap(self.player.health as f32, 0., 100., 0., 150.),
                15.,
            ),
            Color::from_rgb(34, 205, 124),
        )?;

        graphics::draw(ctx, &ammo_rect, DrawParam::default())?;

        graphics::draw(ctx, &hp_rect, DrawParam::default())?;

        graphics::draw(ctx, &cur_ammo_rect, DrawParam::default())?;

        graphics::draw(ctx, &cur_hp_rect, DrawParam::default())?;

        graphics::draw(
            ctx,
            &fish,
            DrawParam::default()
                .dest(Point2::new(
                    ((profile.width() / 2) - 10) as f32,
                    (profile.height() / 3) as f32,
                ))
                .scale(Vector2 { x: 0.7, y: 0.7 }),
        )?;

        let evildoers = &Text::new(
            TextFragment::new(format!(
                "Evildoers {}/{}",
                self.enemies.len(),
                self.total_enemies
            ))
            .font(self.consolas)
            .scale(Scale::uniform(20.)),
        );

        let evildoers_dim = evildoers.dimensions(ctx);

        graphics::draw(
            ctx,
            evildoers,
            DrawParam::default().dest(Point2::new((WIDTH - evildoers_dim.0 as f32) - 40., 20.)),
        )?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        if let Some(t) = self.tics {
            if let Some(_t) = self.tics {
                if self.dim_constant.rate != 0.5 {
                    self.dim_constant.rate = lerp(self.dim_constant.rate, 0.5, 0.1);
                    self.dim_shader.send(ctx, self.dim_constant)?;
                }
            }

            if timer::ticks(ctx) % t as usize == 0 {
                return self.inner_update(ctx);
            }
        } else {
            return self.inner_update(ctx);
        }

        Ok(None)
    }

    pub fn inner_update(&mut self, ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        if self.enemies.len() == 0 {
            self.draw_end_text.3 = true;
            self.can_die = false;

            if self.draw_end_text.1.is_none() {
                self.draw_end_text.1 = Some(timer::ticks(ctx));
            } else if !self.draw_end_text.2 {
                if timer::ticks(ctx) - self.draw_end_text.1.unwrap() > 30 {
                    self.draw_end_text.0 = true;
                    self.draw_end_text.2 = true;
                }
            } else {
                self.tics = Some(1);

                if self.dim_constant.rate != 0.0 {
                    self.dim_constant.rate = lerp(self.dim_constant.rate, 0.0, 0.1);
                    self.dim_shader.send(ctx, self.dim_constant)?;
                }
            }
        }

        let ferris_pos_x = self.player.pos_x;
        let ferris_pos_y = self.player.pos_y;

        let ferris = self.asset_manager.get_image("Some(ferris).png");

        let mut ferris_is_falling_down = true;

        for tile in &mut self.ground {
            // AABB
            if ferris_pos_x + ferris.width() as f32 >= tile.position().pos_start.x
                && tile.position().pos_end.x >= ferris_pos_x
                && ferris_pos_y + ferris.height() as f32 >= tile.position().pos_start.y
                && tile.position().pos_end.y <= ferris_pos_y
            {
                ferris_is_falling_down = false;

                break;
            }
        }

        self.player.update(ferris_is_falling_down);

        self.camera
            .move_to(Vec2::new(self.player.pos_x, self.player.pos_y));

        if self.player.pos_y < -800. {
            if self.can_die {
                return Ok(Some(Screen::Dead));
            }
        }

        for i in 0..self.enemies.len() {
            let go = &mut self.enemies[i];

            go.update(&self.player);

            let mut done: bool = false;

            for j in 0..self.player_bullets.len() {
                let fish = &self.player_bullets[j];

                if go
                    .position()
                    .is_touching(fish.position().pos_start.x, fish.position().pos_start.y)
                {
                    let mut explode_sound = self
                        .asset_manager
                        .get_sound("Some(explode).mp3")
                        .lock()
                        .unwrap();

                    self.particles.push((
                        ParticleSystemBuilder::new(ctx)
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
                            .emission_shape(EmissionShape::Circle(
                                mint::Point2 { x: 0.0, y: 0.0 },
                                100.0,
                            ))
                            .build(),
                        go.position().pos_start.x,
                        -HEIGHT2 + 70.,
                        0,
                    ));

                    explode_sound.play().expect("Cannot play Some(explode).mp3");

                    self.enemies.remove(i);
                    self.player_bullets.remove(j);

                    done = true;

                    break;
                }
            }

            if done {
                let cam_loc = self.camera.location();
                let org_pos = cam_loc.data.as_slice();

                self.elapsed_shake = Some((0., Vec2::new(org_pos[0], org_pos[1]), 3.));
                self.camera_shakeke();

                break;
            }
        }

        for cloud in &mut self.clouds {
            cloud.update(ctx, &self.asset_manager);
        }

        for i in 0..self.barrels.len() {
            let barrel_position = self.barrels[i].position();

            let mut done: bool = false;

            for fish in &self.player_bullets {
                if barrel_position
                    .is_touching(fish.position().pos_start.x, fish.position().pos_start.y)
                {
                    let mut explode_sound = self
                        .asset_manager
                        .get_sound("Some(explode).mp3")
                        .lock()
                        .unwrap();

                    self.particles.push((
                        ParticleSystemBuilder::new(ctx)
                            .count(500)
                            .emission_rate(200.0)
                            .start_max_age(5.0)
                            .start_size_range(2.0, 15.0)
                            .delta_size(Transition::range(1., 2.))
                            .delta_color(Transition::range(
                                ggez::graphics::Color::from((255, 0, 0)),
                                ggez::graphics::Color::from((255, 255, 0)),
                            ))
                            .emission_shape(EmissionShape::Circle(
                                mint::Point2 { x: 0.0, y: 0.0 },
                                200.0,
                            ))
                            .build(),
                        barrel_position.pos_start.x,
                        -HEIGHT2 + 70.,
                        0,
                    ));

                    explode_sound.play().expect("Cannot play Some(explode).mp3");

                    self.barrels.remove(i);

                    done = true;
                }
            }

            if done {
                let cam_loc = self.camera.location();
                let org_pos = cam_loc.data.as_slice();

                self.elapsed_shake = Some((0., Vec2::new(org_pos[0], org_pos[1]), 5.));
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
            } else {
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

                break;
            }
        }

        for v in &mut self.ui_lerp {
            match v.0.as_str() {
                "ammo" => {
                    self.player.ammo = lerp(self.player.ammo, *v.1, 0.3);
                }

                "health" => {
                    // TODO: Health lerping
                }

                _ => panic!(),
            }
        }

        Ok(None)
    }

    pub fn key_press(&mut self, keycode: KeyCode) -> Option<crate::Screen> {
        match keycode {
            KeyCode::Left => {
                self.player.move_x(self.player.pos_x - 10.);

                self.player.set_direction(Direction::Left);
            }
            KeyCode::Right => {
                self.player.move_x(self.player.pos_x + 10.);

                self.player.set_direction(Direction::Right);
            }
            KeyCode::Space => {
                self.player.go_boom();
            }
            KeyCode::S => {
                let ui_lerp = self.ui_lerp.clone();
                let mut turbofish_shoot = self
                    .asset_manager
                    .get_sound("Some(turbofish_shoot).mp3")
                    .lock()
                    .unwrap();

                if let Some(fish) = self.player.shoot(&self.asset_manager) {
                    turbofish_shoot
                        .play()
                        .expect("Cannot play Some(turbofish_shoot).mp3");

                    let cur_ammo = ui_lerp.get("ammo").unwrap();
                    self.ui_lerp.insert(String::from("ammo"), *cur_ammo - 1.);

                    self.player_bullets.push(fish);
                }
            }
            KeyCode::Up => {
                self.tics = Some(6);
            }
            KeyCode::Key7 => {
                return Some(Screen::Menu);
            }
            KeyCode::Key8 => {
                exit(0);
            }
            _ => (),
        }

        None
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => {
                self.tics = None;
                self.dim_constant.rate = 1.0;
            }

            _ => (),
        }

        self.player.set_direction(Direction::None);
    }

    pub fn camera_shakeke(&mut self) {
        let mut rng = rand::thread_rng();

        let elapsed = self.elapsed_shake.unwrap();
        let magnitude = elapsed.2;

        let x = rng.gen_range(-1.0, 1.0) * magnitude;
        let y = rng.gen_range(-1.0, 1.0) * magnitude;

        self.camera.move_by(Vec2::new(x, y));

        self.elapsed_shake = Some((elapsed.0 + 0.1, elapsed.1, magnitude));
    }
}
