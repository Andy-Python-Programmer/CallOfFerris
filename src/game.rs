use ggez::{Context, GameResult, graphics};

#[allow(dead_code)]

const WIDTH: f32 = crate::WIDTH;
#[allow(dead_code)]

const HEIGHT: f32 = crate::HEIGHT;

pub struct Game {
	pub ferris_borrow_fail: graphics::Image,
	pub pos_y: f32,
}

pub fn draw(game_state: &Game, ctx: &mut Context) -> GameResult<()> {
	graphics::clear(ctx, graphics::BLACK);
	
	graphics::draw(
		ctx,
		&game_state.ferris_borrow_fail,
		(ggez::nalgebra::Point2::new((WIDTH / 2.0) - 80.0, game_state.pos_y),),
	).unwrap();

	graphics::present(ctx)
}
