use graphics::Color;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::DrawParams;
use tetra::math::Vec2;
use tetra::{graphics, Context, ContextBuilder, State, TetraError};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

struct GameState {
    lake: Lake,
}

struct Lake {
    mesh: Mesh,
    draw_params: DrawParams,
}

impl Lake {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let center = Vec2::<f32>::new(0., 0.);
        let radius = WINDOW_HEIGHT / 2.0 * 0.8;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, center, radius)?;
        let lake_center = Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        let draw_params = DrawParams::new()
            .position(lake_center)
            .color(Color::rgb8(0, 0, 255));
        Ok(Self { mesh, draw_params })
    }

    fn draw(&self, ctx: &mut Context) {
        self.mesh.draw(ctx, self.draw_params.clone())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            lake: Lake::new(ctx)?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        graphics::clear(ctx, Color::rgb8(30, 240, 30));

        self.lake.draw(ctx);

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
    .run(GameState::new)
}
