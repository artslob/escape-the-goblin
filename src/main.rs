use graphics::Color;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::DrawParams;
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{graphics, input, Context, ContextBuilder, State, TetraError};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PLAYER_SPEED: f32 = 2.0;

struct GameState {
    lake: Lake,
    player: Player,
}

struct Player {
    mesh: Mesh,
    position: Vec2<f32>,
}

impl Player {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let radius = 5.0;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        let center = Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        Ok(Self {
            mesh,
            position: center,
        })
    }

    fn draw(&self, ctx: &mut Context) {
        let draw_params = DrawParams::new()
            .position(self.position)
            .color(Color::WHITE);
        self.mesh.draw(ctx, draw_params);
    }
}

struct Lake {
    mesh: Mesh,
    draw_params: DrawParams,
}

impl Lake {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let radius = WINDOW_HEIGHT / 2.0 * 0.8;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        let position = Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        let draw_params = DrawParams::new()
            .position(position)
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
            player: Player::new(ctx)?,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        let y = if input::is_key_down(ctx, Key::W) {
            Some(-PLAYER_SPEED)
        } else if input::is_key_down(ctx, Key::S) {
            Some(PLAYER_SPEED)
        } else {
            None
        };
        let x = if input::is_key_down(ctx, Key::A) {
            Some(-PLAYER_SPEED)
        } else if input::is_key_down(ctx, Key::D) {
            Some(PLAYER_SPEED)
        } else {
            None
        };
        let player_move = match (x, y) {
            (Some(x), Some(y)) => {
                let value = (PLAYER_SPEED.powi(2) / 2.0).sqrt();
                Vec2::new(value * x.signum(), value * y.signum())
            }
            (Some(x), None) => Vec2::new(x, 0.0),
            (None, Some(y)) => Vec2::new(0.0, y),
            (None, None) => Vec2::zero(),
        };
        self.player.position += player_move;

        // TODO check player is inside lake?
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        graphics::clear(ctx, Color::rgb8(30, 240, 30));

        self.lake.draw(ctx);
        self.player.draw(ctx);

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
