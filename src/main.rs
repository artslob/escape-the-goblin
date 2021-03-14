use graphics::Color;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::DrawParams;
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{graphics, input, Context, ContextBuilder, State, TetraError};

const PI: f32 = std::f32::consts::PI;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PLAYER_SPEED: f32 = 2.0;
const GOBLIN_SPEED: f32 = PLAYER_SPEED * 4.0;

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
    goblin: Goblin,
}

struct Goblin {
    mesh: Mesh,
    position: Vec2<f32>,
}

impl Goblin {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), Self::radius())?;
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

    fn radius() -> f32 {
        5.0
    }
}

struct Player {
    mesh: Mesh,
    position: Vec2<f32>,
}

impl Player {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), Self::radius())?;
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

    fn radius() -> f32 {
        5.0
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
            goblin: Goblin::new(ctx)?,
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

        // 1. find arc length
        // 2. if length < GOBLIN_SPEED, move goblin to player
        // 3. else move goblin to GOBLIN_SPEED point
        let goblin_vector = self.goblin.position - Lake::center();
        let player_vector = self.player.position - Lake::center();
        let angle = goblin_vector.angle_between(player_vector); // angle in radians

        // https://tutors.com/math-tutors/geometry-help/how-to-find-arc-measure-formula
        let arc_length = Lake::radius() * angle;
        if arc_length <= GOBLIN_SPEED {
            self.goblin.position =
                Lake::center() + player_vector * (Lake::radius() / player_vector.magnitude());
        } else {
            let arc_length = GOBLIN_SPEED;
            let angle = arc_length / Lake::radius();
            // https://www.euclideanspace.com/maths/algebra/vectors/angleBetween/
            // https://stackoverflow.com/questions/21483999/using-atan2-to-find-angle-between-two-vectors
            let angle_sign =
                player_vector.y.atan2(player_vector.x) - goblin_vector.y.atan2(goblin_vector.x);
            let angle_sign = if angle_sign > PI {
                angle_sign - 2. * PI
            } else if angle_sign <= -PI {
                angle_sign + 2. * PI
            } else {
                angle_sign
            };
            let angle = angle * angle_sign.signum();
            // https://en.wikipedia.org/wiki/Rotation_matrix
            let goblin_rotated = Vec2::new(
                goblin_vector.x * angle.cos() - goblin_vector.y * angle.sin(),
                goblin_vector.x * angle.sin() + goblin_vector.y * angle.cos(),
            );
            self.goblin.position = Lake::center() + goblin_rotated;
        }

        let vector_between = self.player.position - self.goblin.position;
        if vector_between.magnitude() < Player::radius() + Goblin::radius() {
            self.result = GameResult::GoblinWins;
        } else if (self.player.position.x - Lake::center().x).powi(2)
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
