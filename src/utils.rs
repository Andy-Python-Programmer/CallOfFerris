use std::{collections::HashMap, sync::Mutex};

use ggez::{
    audio::Source,
    graphics::{Font, Image},
    nalgebra::Point2,
    Context,
};

pub fn lerp(from: f32, to: f32, dt: f32) -> f32 {
    from + dt * (to - from)
}

pub fn remap(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Position {
    pub pos_start: Point2<f32>,
    pub pos_end: Point2<f32>,

    width: u16,
    height: u16
}

impl Position {
    pub fn new(x: f32, y: f32, width: u16, height: u16) -> Self {
        let pos_start = Point2::new(x, y);
        let pos_end = Point2::new(x + width as f32, y - height as f32);

        Self { pos_start, pos_end, width, height }
    }

    pub fn is_touching(&self, x: f32, y: f32) -> bool {
        if x >= self.pos_start.x
            && x <= self.pos_end.x
            && y <= self.pos_start.y
            && y >= self.pos_end.y
        {
            true
        } else {
            false
        }
    }

    pub fn move_by(&mut self, axis: &str, by: f32) {
        match axis {
            "x+" => {
                self.pos_start.x += by;
                self.pos_end.x += by;
            }

            "y+" => {
                self.pos_start.y += by;
                self.pos_end.y += by;
            },

            "x-" => {
                self.pos_start.x -= by;
                self.pos_end.x -= by;
            }

            "y-" => {
                self.pos_start.y -= by;
                self.pos_end.y -= by;
            },

            _ => panic!(),
        }
    }

    #[allow(dead_code)]
    pub fn move_to(&mut self, axis: &str, to: f32) {
        match axis {
            "x" => {
                self.pos_start.x = to;
                self.pos_end.x = to + self.width as f32;
            },

            "y" => {
                self.pos_start.y = to;
                self.pos_end.y = to - self.height as f32;
            }

            _ => panic!()
        }
    }
}
enum Asset {
    Image(Image),
    Font(Font),
    Audio(Mutex<Source>),
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

    pub fn load_image(&mut self, ctx: &mut Context, filename: &str) {
        self.assets.insert(
            filename.to_string(),
            Asset::Image(
                Image::new(ctx, format!("/images/{}", filename))
                    .expect(format!("Cannot load {}", filename).as_str()),
            ),
        );
    }

    pub fn load_font(&mut self, ctx: &mut Context, filename: &str) {
        self.assets.insert(
            filename.to_string(),
            Asset::Font(
                Font::new(ctx, format!("/fonts/{}", filename))
                    .expect(format!("Cannot load {}", filename).as_str()),
            ),
        );
    }

    pub fn load_sound(&mut self, ctx: &mut Context, filename: &str) {
        self.assets.insert(
            filename.to_string(),
            Asset::Audio(Mutex::new(
                Source::new(ctx, format!("/audio/{}", filename))
                    .expect(format!("Cannot load {}", filename).as_str()),
            )),
        );
    }

    pub fn get_image(&self, filename: &str) -> Image {
        match self.assets.get(&filename.to_string()).unwrap() {
            Asset::Image(image) => {
                return image.to_owned();
            }
            _ => panic!(),
        }
    }

    pub fn get_font(&self, filename: &str) -> Font {
        match self.assets.get(&filename.to_string()).unwrap() {
            Asset::Font(font) => {
                return font.to_owned();
            }
            _ => panic!(),
        }
    }

    pub fn get_sound(&self, filename: &str) -> &Mutex<Source> {
        match self.assets.get(&filename.to_string()).unwrap() {
            Asset::Audio(audio) => {
                return audio;
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use super::Position;

    #[test]
    fn new_position() {
        let pos = Position::new(10.0, 10.0, 10, 10);

        assert!(
            pos.pos_start == Point2::new(10.0, 10.0),
            pos.pos_end == Point2::new(20.0, 20.0)
        );
    }

    #[test]
    fn colliding() {
        let pos = Position::new(896.0, -110.0, 100, 128);

        assert!(pos.is_touching(902.0, -194.0))
    }
}
