use ggez::{
    graphics::Color,
    graphics::{self, Scale, Text, TextFragment},
    Context, GameResult,
};

pub struct Death {
    consolas: graphics::Font,
    ferris_planet: graphics::Image,
}

impl Death {
    pub fn spawn(ctx: &mut Context) -> Self {
        Self {
            consolas: graphics::Font::new(ctx, "/fonts/Consolas.ttf").unwrap(),
            ferris_planet: graphics::Image::new(ctx, "/images/ferris_planet.png").unwrap(),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let dead = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "YOU DEAD".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            scale: Some(Scale::uniform(35.0)),
            font: Some(self.consolas),
            color: Some(Color::new(1.0, 80.0 / 255.0, 76.0 / 255.0, 1.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });

        let unsafe_dead = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "unsafe".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            scale: Some(Scale::uniform(30.0)),
            font: Some(self.consolas),
            color: Some(Color::new(74.0 / 255.0, 129.0 / 255.0, 191.0 / 255.0, 1.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });

        let unsafe_dead_block_start = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "{".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            scale: Some(Scale::uniform(30.0)),
            font: Some(self.consolas),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });

        let unsafe_dead_block_func = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "dead()".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            scale: Some(Scale::uniform(30.0)),
            font: Some(self.consolas),
            color: Some(Color::new(214.0 / 255.0, 208.0 / 255.0, 132.0 / 255.0, 1.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });

        let unsafe_dead_block_end = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "}".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            scale: Some(Scale::uniform(30.0)),
            font: Some(self.consolas),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });

        graphics::draw(
            ctx,
            &dead,
            (ggez::mint::Point2 {
                x: (crate::WIDTH / 2.0) - 60.0,
                y: 40.0,
            },),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead,
            (ggez::mint::Point2 {
                x: (crate::WIDTH / 2.0) - 200.0,
                y: 200.0,
            },),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead_block_start,
            (ggez::mint::Point2 {
                x: (crate::WIDTH / 2.0) - 90.0,
                y: 200.0,
            },),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead_block_func,
            (ggez::mint::Point2 {
                x: (crate::WIDTH / 2.0) - 125.0,
                y: 260.0,
            },),
        )?;

        graphics::draw(
            ctx,
            &unsafe_dead_block_end,
            (ggez::mint::Point2 {
                x: (crate::WIDTH / 2.0) - 200.0,
                y: 300.0,
            },),
        )?;

        graphics::draw(
            ctx,
            &self.ferris_planet,
            (ggez::nalgebra::Point2::new(
                (crate::WIDTH / 2.0) - 10.0,
                240.0,
            ),),
        )
        .unwrap();

        graphics::present(ctx)
    }

    pub fn update(&self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
