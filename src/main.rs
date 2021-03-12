use graphics::Color;
use tetra::{graphics, Context, ContextBuilder, State, TetraError};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

struct GameState {}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        graphics::clear(ctx, Color::rgb8(30, 240, 30));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new(
        "Escape the Goblin!",
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
    )
    .build()?
    .run(|_| Ok(GameState {}))
}
