use graphics::Color;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text, VectorFontBuilder};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::{Key, MouseButton};
use tetra::math::Vec2;
use tetra::{graphics, input, Context, ContextBuilder, State, TetraError};

const PI: f32 = std::f32::consts::PI;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PLAYER_SPEED: f32 = 2.0;
const GOBLIN_SPEED: f32 = PLAYER_SPEED * 4.0;

// TODO allow resize window

enum GameResult {
    Playing,
    Ended { text: Text, background_color: Color },
}

struct HelpingCircle {
    mesh: Mesh,
    position: Vec2<f32>,
}

impl HelpingCircle {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mesh = Mesh::circle(ctx, ShapeStyle::Stroke(1.0), Vec2::zero(), Self::radius())?;
        Ok(Self {
            mesh,
            position: Lake::center(),
        })
    }

    fn draw(&self, ctx: &mut Context) {
        let draw_params = DrawParams::new()
            .position(self.position)
            .color(Color::WHITE);
        self.mesh.draw(ctx, draw_params);
    }

    fn radius() -> f32 {
        Lake::radius() / (GOBLIN_SPEED / PLAYER_SPEED)
    }
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

struct GameState {
    result: GameResult,
    lake: Lake,
    player: Player,
    goblin: Goblin,
    helping_circle: HelpingCircle,
    player_wins_text: Text,
    goblin_wins_text: Text,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let font_builder = VectorFontBuilder::new("./fonts/NewTegomin-Regular.ttf")?;
        let font = font_builder.with_size(ctx, 64.0)?;
        let player_wins_text = Text::new("You win! Congrats!", font.clone());
        let goblin_wins_text = Text::new("Goblin wins!", font.clone());
        Ok(Self {
            result: GameResult::Playing,
            lake: Lake::new(ctx)?,
            player: Player::new(ctx)?,
            goblin: Goblin::new(ctx)?,
            helping_circle: HelpingCircle::new(ctx)?,
            player_wins_text,
            goblin_wins_text,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        if let GameResult::Ended { .. } = self.result {
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

        // TODO count only mouse or keyboard, but not both at same time
        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            let mouse_position = input::get_mouse_position(ctx);
            let difference = mouse_position - self.player.position;
            let magnitude = difference.magnitude();
            let position = if magnitude > PLAYER_SPEED {
                self.player.position + difference * PLAYER_SPEED / magnitude
            } else {
                mouse_position
            };
            self.player.position = position
        }

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
            self.result = GameResult::Ended {
                text: self.goblin_wins_text.clone(),
                background_color: Color::rgb8(240, 30, 30),
            };
        } else if (self.player.position.x - Lake::center().x).powi(2)
            + (self.player.position.y - Lake::center().y).powi(2)
            > Lake::radius().powi(2)
        {
            self.result = GameResult::Ended {
                text: self.player_wins_text.clone(),
                background_color: Color::rgb8(30, 144, 255),
            };
        };

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        match &mut self.result {
            GameResult::Playing => {
                graphics::clear(ctx, Color::rgb8(30, 240, 30));
                self.lake.draw(ctx);
                self.player.draw(ctx);
                self.goblin.draw(ctx);
                self.helping_circle.draw(ctx);
            }
            GameResult::Ended {
                text,
                background_color,
            } => {
                graphics::clear(ctx, *background_color);
                let position = match text.get_bounds(ctx) {
                    Some(rect) => Vec2::new(
                        (WINDOW_WIDTH - rect.width) / 2.,
                        (WINDOW_HEIGHT - rect.height) / 2.,
                    ),
                    None => Vec2::zero(),
                };
                text.draw(
                    ctx,
                    DrawParams::new().position(position).color(Color::WHITE),
                )
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
    .show_mouse(true)
    .build()?
    .run(GameState::new)
}
