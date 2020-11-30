use ggez::{Context, GameResult, event::KeyCode, graphics::{self, Color, Scale, Text, TextFragment}};
use std::process::exit;

pub struct Menu {
    pub consolas: graphics::Font,
    pub ferris_borrow_fail: graphics::Image
}

impl Menu {
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
    
        let call_of_ferris_text = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "Call of Ferris".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            font: Some(self.consolas),
            scale: Some(Scale::uniform(33.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });
    
        let ownership_war = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "Ownership War".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            font: Some(self.consolas),
            scale: Some(Scale::uniform(14.0)),
    
            color: Some(Color::new(1.0, 80.0 / 255.0, 76.0 / 255.0, 1.0)),
            ..Default::default()
        });
    
        graphics::draw(
            ctx,
            &self.ferris_borrow_fail,
            (ggez::nalgebra::Point2::new((crate::WIDTH / 2.0) - 85.0, 100.0),),
        )
        .unwrap();
        graphics::draw(
            ctx,
            &call_of_ferris_text,
            (ggez::nalgebra::Point2::new((crate::WIDTH / 2.0) - 130.0, 250.0),),
        )
        .unwrap();
        graphics::draw(
            ctx,
            &ownership_war,
            (ggez::nalgebra::Point2::new((crate::WIDTH / 2.0) - 50.0, 300.0),),
        )
        .unwrap();
    
        let play_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new((crate::WIDTH / 2.0) - 320.0, 410.0, 160.0, 60.0),
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;
    
        let dirty_pointer = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new((crate::WIDTH / 2.0) + 180.0, 410.0, 160.0, 60.0),
            [4.0 / 255.0, 129.0  / 255.0, 191.0 / 255.0, 1.0].into(),
        )?;
    
        let play_borrow = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "& to play".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            font: Some(self.consolas),
            scale: Some(Scale::uniform(25.0)),
    
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });
    
        let dirty_pointer_quit = Text::new(TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: "* to quit".to_string(),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            font: Some(self.consolas),
            scale: Some(Scale::uniform(25.0)),
    
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
        });
    
        graphics::draw(ctx, &play_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(
            ctx,
            &dirty_pointer,
            (ggez::mint::Point2 { x: 0.0, y: 0.0 },),
        )?;
    
        graphics::draw(
            ctx,
            &play_borrow,
            (ggez::mint::Point2 {
                x: (crate::WIDTH / 2.0) - 300.0,
                y: 430.0,
            },),
        )?;
        graphics::draw(
            ctx,
            &dirty_pointer_quit,
            (ggez::mint::Point2 {
                x: (crate::WIDTH / 2.0) + 200.0,
                y: 430.0,
            },),
        )?;
    
        graphics::present(ctx)
    }    

    pub fn key_press(&self, keycode: KeyCode) -> Option<crate::Screen> {
        if keycode == KeyCode::Key7 {
           return Some(crate::Screen::Play);
        }

        else if keycode == KeyCode::Key8 {
            exit(0);
        }

        return None;
    }
}