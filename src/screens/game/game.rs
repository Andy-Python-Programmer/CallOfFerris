use std::{collections::HashMap, process::exit, rc::Rc, sync::Mutex};

use ggez::{
    audio::SoundSource,
    event::KeyCode,
    graphics::{self, Color, DrawParam, Drawable, Shader, Text},
    mint,
    nalgebra::Point2,
    timer, Context, GameResult,
};
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::{GlBackendSpec, Scale, ShaderGeneric, TextFragment};
use mint::Vector2;
use rand::Rng;

use crate::{
    game::components::{
        bullet::{PlayerWeapon, WeaponType},
        cloud::Cloud,
        player::Direction,
    },
    game::map::Map,
    game::physics::Physics,
    play,
    utils::{lerp, remap, AssetManager, ParticleSystem},
    Screen,
};

use gfx::*;

gfx_defines! {
    constant Dim {
        rate: f32 = "u_Rate",
    }
}

pub struct Game {
    /// The game map.
    map: Map,
    /// Physics system for the game.
    physics: Physics,
    /// Camera to see the world.
    camera: Camera,

    // TODO: Refactor the rest of the fields
    clouds: Vec<Cloud>,

    /// Reference to the asset manager.
    asset_manager: Rc<AssetManager>,

    elapsed_shake: Option<(f32, Vec2, f32)>,
    tics: Option<i32>,
    particles: Vec<ParticleSystem>,
    ui_lerp: HashMap<String, f32>,

    dim_shader: ShaderGeneric<GlBackendSpec, Dim>,
    dim_constant: Dim,

    draw_end_text: (bool, Option<usize>, bool, bool), // Thread Sleeped?, Current Iters, Done?, Win?
    can_die: bool,
}

impl Game {
    pub fn create(ctx: &mut Context, asset_manager: Rc<AssetManager>) -> Mutex<Self> {
        let (width, height) = graphics::drawable_size(ctx);

        let mut camera = Camera::new(width as u32, height as u32, width, height);

        let mut rng = rand::thread_rng();

        let mut physics = Physics::new();
        let mut map = Map::parse(ctx, "01", &mut physics, &asset_manager);

        let mut clouds = vec![];

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

        let mut ui_lerp = HashMap::new();

        ui_lerp.insert(String::from("ammo"), map.player.ammo as f32);
        ui_lerp.insert(String::from("health"), map.player.health as f32);
        ui_lerp.insert(String::from("using"), map.using.as_ref().unwrap().1);

        map.player.init(&mut physics);

        camera.move_to(Vec2::new(
            map.player.position(&mut physics).x,
            map.player.position(&mut physics).y,
        ));

        for _ in 0..rng.gen_range(5..=7) {
            clouds.push(Cloud::new(
                rng.gen_range(0. ..=width),
                rng.gen_range(10. ..=40.),
                rng.gen_range(0.1..=0.3),
                rng.gen_range(10. ..=35.),
            ));
        }

        Mutex::new(Self {
            map,
            physics,

            clouds,

            asset_manager,

            camera,

            elapsed_shake: None,
            tics: None,
            particles: vec![],
            ui_lerp,

            dim_shader,
            dim_constant,
            draw_end_text: (false, None, false, false),
            can_die: true,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<Option<Screen>> {
        let (width, height) = graphics::drawable_size(ctx);

        let consolas = self.asset_manager.get_font("Consolas.ttf");

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
                        .font(consolas)
                        .scale(Scale::uniform(50.)),
                );

                let end_dimensions = end_frag.dimensions(ctx);

                graphics::draw(
                    ctx,
                    end_frag,
                    DrawParam::default().dest(Point2::new(
                        (width / 2.0) - (end_dimensions.0 / 2) as f32,
                        50.0,
                    )),
                )?;

                // End quote
                for line in self
                    .map
                    .end
                    .as_ref()
                    .unwrap()
                    .split("\\n")
                    .collect::<Vec<_>>()
                {
                    let end_frag = &Text::new(TextFragment::new(line).font(consolas));

                    let end_dimensions = end_frag.dimensions(ctx);

                    graphics::draw(
                        ctx,
                        end_frag,
                        DrawParam::default().dest(Point2::new(
                            (width / 2.0) - (end_dimensions.0 / 2) as f32,
                            height / 2. + draw_pos,
                        )),
                    )?;

                    draw_pos += 20.0;
                }

                // Press & to go to menu screen
                let menu_rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        (width / 2.) + 20.,
                        (height / 2.) + (draw_pos * 2.),
                        220.0,
                        40.0,
                    ),
                    [36.0 / 255.0, 36.0 / 255.0, 36.0 / 255.0, 0.9].into(),
                )?;

                let menu_rect_dim = menu_rect.dimensions(ctx).unwrap();

                let menu_frag_to =
                    &Text::new(TextFragment::new("Press & go to the").font(consolas));

                let menu_screen = &Text::new(
                    TextFragment::new("MENU SCREEN")
                        .font(consolas)
                        .scale(Scale::uniform(20.0)),
                );

                graphics::draw(ctx, &menu_rect, DrawParam::default())?;
                graphics::draw(
                    ctx,
                    menu_frag_to,
                    DrawParam::default().dest(Point2::new(
                        (width / 2.) + 20.,
                        ((height / 2.) + (draw_pos * 2.)) - 20.0,
                    )),
                )?;

                graphics::draw(
                    ctx,
                    menu_screen,
                    DrawParam::default().dest(Point2::new(
                        (width / 2.) + 70.,
                        ((height / 2.) + (draw_pos * 2.)) + 12.0,
                    )),
                )?;

                // Press * to quit
                let quit_rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        ((width / 2.) - menu_rect_dim.w) - 20.0,
                        (height / 2.) + (draw_pos * 2.),
                        220.0,
                        40.0,
                    ),
                    [36.0 / 255.0, 36.0 / 255.0, 36.0 / 255.0, 0.9].into(),
                )?;

                let quit_frag_to = &Text::new(TextFragment::new("Press * to").font(consolas));

                let press_quit = &Text::new(
                    TextFragment::new("QUIT")
                        .font(consolas)
                        .scale(Scale::uniform(20.)),
                );

                graphics::draw(ctx, &quit_rect, DrawParam::default())?;
                graphics::draw(
                    ctx,
                    quit_frag_to,
                    DrawParam::default().dest(Point2::new(
                        ((width / 2.) - menu_rect_dim.w) - 20.,
                        ((height / 2.) + (draw_pos * 2.)) - 20.,
                    )),
                )?;

                graphics::draw(
                    ctx,
                    press_quit,
                    DrawParam::default().dest(Point2::new(
                        (((width / 2.) - menu_rect_dim.w) - 20.) + 90.,
                        (((height / 2.) + (draw_pos * 2.)) - 20.) + 30.,
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
        for tile in &mut self.map.ground {
            tile.draw(ctx, &self.camera, &mut self.physics, &self.asset_manager)?;
        }

        // Enemies
        for enemy in &mut self.map.enemies {
            enemy.draw(ctx, &self.camera, &mut self.physics, &self.asset_manager)?;
        }

        // Barrel
        for boom in &mut self.map.barrels {
            boom.draw(ctx, &self.camera, &mut self.physics, &self.asset_manager)?;
        }

        // Player
        self.map
            .player
            .draw(ctx, &self.camera, &mut self.physics, &self.asset_manager)?;

        // Particles
        for sys in &mut self.particles {
            sys.draw(ctx, &mut self.physics, &mut self.camera)?;
        }

        // User Profile, etc..
        self.draw_ui(ctx)?;

        #[cfg(feature = "debug")]
        self.physics.draw_colliders(ctx, &self.camera)?;

        Ok(())
    }

    fn draw_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (width, _) = graphics::drawable_size(ctx);

        let profile = self.asset_manager.get_image("Some(profile).png");
        let fish = self.asset_manager.get_image("Some(fish).png");

        let consolas = self.asset_manager.get_font("Consolas.ttf");

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
                remap(self.map.player.ammo as f32, 0., 10., 0., 150.),
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
                remap(self.map.player.health as f32, 0., 100., 0., 150.),
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
                self.map.enemies.len(),
                self.map.total_enemies
            ))
            .font(consolas)
            .scale(Scale::uniform(20.)),
        );

        let evildoers_dim = evildoers.dimensions(ctx);

        graphics::draw(
            ctx,
            evildoers,
            DrawParam::default().dest(Point2::new((width - evildoers_dim.0 as f32) - 40., 20.)),
        )?;

        let info = &Text::new(
            TextFragment::new(format!("Using {}", self.map.using.as_ref().unwrap().0))
                .font(consolas)
                .color([1.0, 1.0, 1.0, self.map.using.as_ref().unwrap().1].into()),
        );

        let info_dim = info.dimensions(ctx);

        graphics::draw(
            ctx,
            info,
            DrawParam::default().dest(Point2::new((width / 2.) - (info_dim.0 / 2) as f32, 150.)),
        )?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        if let Some(t) = self.tics {
            if self.tics.is_some() && self.dim_constant.rate != 0.5 {
                self.dim_constant.rate = lerp(self.dim_constant.rate, 0.5, 0.1);
                self.dim_shader.send(ctx, self.dim_constant)?;
            }

            if timer::ticks(ctx) % t as usize == 0 {
                return self.inner_update(ctx);
            }
        } else {
            return self.inner_update(ctx);
        }

        Ok(None)
    }

    fn inner_update(&mut self, ctx: &mut Context) -> GameResult<Option<crate::Screen>> {
        let (_, height) = graphics::drawable_size(ctx);

        // Take a time step in our physics world!
        self.physics.step();

        // Update our player
        self.map.player.update(ctx, &mut self.physics);
        self.camera.move_to(Vec2::new(
            self.map.player.position(&mut self.physics).x,
            self.map.player.position(&mut self.physics).y,
        ));

        // Update our lovely clouds
        for cloud in &mut self.clouds {
            cloud.update(ctx);
        }

        if self.map.enemies.is_empty() {
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

        if self.map.player.position(&mut self.physics).y > height && self.can_die {
            return Ok(Some(Screen::Dead));
        }

        for id in 0..self.map.enemies.len() {
            let enemy = &mut self.map.enemies[id];

            if enemy.update(
                &mut self.physics,
                &self.asset_manager,
                &mut self.particles,
                &mut self.map.player,
            ) {
                self.map.enemies.remove(id);
                let cam_loc = self.camera.location();
                let org_pos = cam_loc.data.as_slice();

                self.elapsed_shake = Some((0., Vec2::new(org_pos[0], org_pos[1]), 3.));
                self.camera_shakeke();

                break;
            };
        }

        for id in 0..self.map.barrels.len() {
            if self.map.barrels[id].update(
                &mut self.physics,
                &self.asset_manager,
                &mut self.particles,
                &mut self.map.player,
            ) {
                self.map.barrels.remove(id);
                let cam_loc = self.camera.location();
                let org_pos = cam_loc.data.as_slice();

                self.elapsed_shake = Some((0., Vec2::new(org_pos[0], org_pos[1]), 5.));
                self.camera_shakeke();
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

        for id in 0..self.particles.len() {
            let sys = &mut self.particles[id];

            if sys.update(ctx, &mut self.physics) {
                self.particles.remove(id);

                break;
            }
        }

        for v in &mut self.ui_lerp {
            match v.0.as_str() {
                "ammo" => {
                    if self.map.player.ammo <= 0.0 {
                        self.map.player.ammo = 0.0;
                    } else {
                        self.map.player.ammo = lerp(self.map.player.ammo, *v.1, 0.3);
                    }
                }

                "health" => {
                    // TODO: Health lerping
                }

                "using" => {
                    self.map.using.as_mut().unwrap().1 =
                        lerp(self.map.using.as_mut().unwrap().1, 0.0, 0.05);
                }

                _ => panic!(),
            }
        }

        Ok(None)
    }

    pub fn key_press(&mut self, keycode: KeyCode) -> Option<crate::Screen> {
        match keycode {
            KeyCode::S => {
                let ui_lerp = self.ui_lerp.clone();
                let turbofish_shoot = self.asset_manager.get_sound("Some(turbofish_shoot).mp3");

                if let Some(bullet) =
                    self.map
                        .player
                        .shoot(&mut self.physics, &self.asset_manager, &self.map.weapon)
                {
                    play!(turbofish_shoot);

                    if let PlayerWeapon::Turbofish(_fish) = &bullet {
                        let cur_ammo = ui_lerp.get("ammo").unwrap();
                        self.ui_lerp.insert(String::from("ammo"), *cur_ammo - 1.);
                    }

                    self.map.player.weapons.push(bullet);
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
            KeyCode::Down => match self.map.using.as_ref().unwrap().0.as_str() {
                "Turbofish Gun" => {
                    self.map.using = Some((String::from("Grappling Gun"), 1.0));
                    self.map.weapon = WeaponType::Grappling;
                }

                "Grappling Gun" => {
                    self.map.using = Some((String::from("Turbofish Gun"), 1.0));
                    self.map.weapon = WeaponType::Turbofish;
                }

                _ => {
                    panic!()
                }
            },
            _ => (),
        }

        None
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        if keycode == KeyCode::Up {
            self.tics = None;
            self.dim_constant.rate = 1.0;
        }
        self.map.player.set_direction(Direction::None);
    }

    /// Give the camera a shakey shakey.
    fn camera_shakeke(&mut self) {
        let mut rng = rand::thread_rng();

        let elapsed = self.elapsed_shake.unwrap();
        let magnitude = elapsed.2;

        let x = rng.gen_range(-1.0..=1.0) * magnitude;
        let y = rng.gen_range(-1.0..=1.0) * magnitude;

        self.camera.move_by(Vec2::new(x, y));

        self.elapsed_shake = Some((elapsed.0 + 0.1, elapsed.1, magnitude));
    }
}
