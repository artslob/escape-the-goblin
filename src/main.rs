use graphics::Color;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::DrawParams;
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{graphics, input, Context, ContextBuilder, State, TetraError};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PLAYER_SPEED: f32 = 2.0;

#[derive(Eq, PartialEq)]
enum GameResult {
    Playing,
    PlayerWins,
    GoblinWins,
}

struct GameState {
    result: GameResult,
    lake: Lake,
    player: Player,
    goblin: Godlin,
}

struct Godlin {
    mesh: Mesh,
    position: Vec2<f32>,
}

impl Godlin {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let radius = 5.0;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        let center = Vec2::new(WINDOW_WIDTH / 2.0, Lake::center().y - Lake::radius());
        Ok(Self {
            mesh,
            position: center,
        })
    }

    fn draw(&self, ctx: &mut Context) {
        let draw_params = DrawParams::new().position(self.position).color(Color::RED);
        self.mesh.draw(ctx, draw_params);
    }
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
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), Self::radius())?;
        let draw_params = DrawParams::new()
            .position(Self::center())
            .color(Color::rgb8(0, 0, 255));
        Ok(Self { mesh, draw_params })
    }

    fn draw(&self, ctx: &mut Context) {
        self.mesh.draw(ctx, self.draw_params.clone())
    }

    fn center() -> Vec2<f32> {
        Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0)
    }

    fn radius() -> f32 {
        WINDOW_HEIGHT / 2.0 * 0.8
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            result: GameResult::Playing,
            lake: Lake::new(ctx)?,
            player: Player::new(ctx)?,
            goblin: Godlin::new(ctx)?,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        if self.result != GameResult::Playing {
            return Ok(());
        }

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

        if (self.player.position.x - Lake::center().x).powi(2)
            + (self.player.position.y - Lake::center().y).powi(2)
            > Lake::radius().powi(2)
        {
            self.result = GameResult::PlayerWins;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        match self.result {
            GameResult::Playing => {
                graphics::clear(ctx, Color::rgb8(30, 240, 30));
                self.lake.draw(ctx);
                self.player.draw(ctx);
                self.goblin.draw(ctx);
            }
            GameResult::PlayerWins => {
                graphics::clear(ctx, Color::rgb8(30, 144, 255));
            }
            GameResult::GoblinWins => {
                graphics::clear(ctx, Color::rgb8(240, 30, 30));
            }
        }

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
