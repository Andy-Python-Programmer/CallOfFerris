use std::{borrow::Cow, collections::HashMap, error::Error, io::Read, sync::Mutex};

use ggez::{
    audio::Source,
    graphics::{self, Color, DrawMode, Font, Image, Mesh},
    nalgebra::Point2,
    timer, Context, GameResult,
};

use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};
use graphics::DrawParam;
use nphysics2d::{
    algebra::Velocity2,
    nalgebra as na,
    ncollide2d::shape::{Ball, ShapeHandle},
    object::{BodyPartHandle, ColliderDesc, DefaultBodyHandle, RigidBodyDesc},
};
use rand::Rng;

use crate::game::physics::{isometry_to_point, point_to_isometry, ObjectData, Physics};

pub type FerrisResult<T> = Result<T, Box<dyn Error>>;

pub fn lerp(from: f32, to: f32, dt: f32) -> f32 {
    from + dt * (to - from)
}

pub fn remap(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2
}

#[macro_export]
macro_rules! play {
    ($exp:expr) => {
        let mut sound = $exp
            .lock()
            .expect(format!("Cannot load {}.mp3", stringify!($exp)).as_str());

        sound
            .play()
            .expect(format!("Cannot play {}.mp3", stringify!($exp)).as_str());
    };
}

enum Asset {
    Image(Image),
    Font(Font),
    Audio(Mutex<Source>),
    File(String),
}

pub struct AssetManager {
    assets: HashMap<String, Asset>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn load_image(&mut self, ctx: &mut Context, filename: Cow<'_, str>) {
        self.assets.insert(
            filename.to_string(),
            Asset::Image(
                Image::new(ctx, format!("/images/{}", filename))
                    .unwrap_or_else(|_| panic!("Cannot load {}", filename)),
            ),
        );
    }

    pub fn load_font(&mut self, ctx: &mut Context, filename: Cow<'_, str>) {
        self.assets.insert(
            filename.to_string(),
            Asset::Font(
                Font::new(ctx, format!("/fonts/{}", filename))
                    .unwrap_or_else(|_| panic!("Cannot load {}", filename)),
            ),
        );
    }

    pub fn load_sound(&mut self, ctx: &mut Context, filename: Cow<'_, str>) {
        self.assets.insert(
            filename.to_string(),
            Asset::Audio(Mutex::new(
                Source::new(ctx, format!("/audio/{}", filename))
                    .unwrap_or_else(|_| panic!("Cannot load {}", filename)),
            )),
        );
    }

    pub fn load_file(&mut self, ctx: &mut Context, folder: &str, filename: Cow<'_, str>) {
        let path = format!("/{}/{}", folder, filename);

        let mut file = ggez::filesystem::open(ctx, &path).unwrap();
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        self.assets.insert(path, Asset::File(buffer));
    }

    pub fn get_image(&self, filename: &str) -> Image {
        match self.assets.get(&filename.to_string()).unwrap() {
            Asset::Image(image) => image.to_owned(),
            _ => panic!(),
        }
    }

    pub fn get_font(&self, filename: &str) -> Font {
        match self.assets.get(&filename.to_string()).unwrap() {
            Asset::Font(font) => font.to_owned(),
            _ => panic!(),
        }
    }

    pub fn get_sound(&self, filename: &str) -> &Mutex<Source> {
        match self.assets.get(&filename.to_string()).unwrap() {
            Asset::Audio(audio) => audio,
            _ => panic!(),
        }
    }

    pub fn get_file(&self, filename: &str) -> String {
        match self.assets.get(&filename.to_string()).unwrap() {
            Asset::File(file) => file.to_owned(),
            _ => panic!(),
        }
    }
}

pub struct ParticleSystem {
    particles: Vec<DefaultBodyHandle>,
    lifetime: f32,
}

impl ParticleSystem {
    const PARTICLE_JUICE: f32 = 300.0;

    pub fn new(
        physics: &mut Physics,
        amount: usize,
        min: na::Point2<f32>,
        max: na::Point2<f32>,
    ) -> Self {
        let rng = &mut rand::thread_rng();
        let mut particles = vec![];

        for _ in 0..amount {
            let position =
                na::Point2::new(rng.gen_range(min.x, max.x), rng.gen_range(min.y, max.y));
            let mut body = RigidBodyDesc::new()
                .mass(0.1)
                .position(point_to_isometry(position))
                .build();

            body.set_velocity(Velocity2::linear(
                rng.gen_range(-Self::PARTICLE_JUICE, Self::PARTICLE_JUICE),
                rng.gen_range(-Self::PARTICLE_JUICE, Self::PARTICLE_JUICE),
            ));

            let g = rng.gen_range(0, 255);

            let handle = physics.create_rigid_body(body);
            let shape = ShapeHandle::new(Ball::new(2.0));
            let collider = ColliderDesc::new(shape)
                .user_data(ObjectData::Particle(Color::from_rgb(255, g, 0)))
                .build(BodyPartHandle(handle, 0));

            physics.create_collider(collider);

            particles.push(handle);
        }

        Self {
            particles,
            lifetime: 2.0,
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        physics: &mut Physics,
        camera: &mut Camera,
    ) -> GameResult<()> {
        for particle in &self.particles {
            let body = physics.get_rigid_body(*particle);
            let position = isometry_to_point(body.position());
            let color = physics.get_user_data(*particle).get_particle_data();

            let color = Color::new(color.r, color.g, color.b, self.lifetime);

            let particle_mesh = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Point2::new(0.0, 0.0),
                2.0,
                0.1,
                color,
            )?;

            let camera_pos = camera.calculate_dest_point(Vec2::new(position.x, position.y));

            graphics::draw(
                ctx,
                &particle_mesh,
                DrawParam::default()
                    .dest(Point2::new(camera_pos.x, camera_pos.y))
                    .offset(Point2::new(0.5, 0.5)),
            )?;
        }

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, physics: &mut Physics) -> bool {
        self.lifetime -= timer::delta(ctx).as_secs_f32();

        if self.lifetime <= 0.0 {
            for id in 0..self.particles.len() {
                // Destroy the particle from the world. The lifetime of the particle has been ended
                physics.destroy_body(self.particles[id]);
            }

            true
        } else {
            false
        }
    }
}
