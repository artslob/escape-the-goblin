use graphics::Color;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Text, VectorFontBuilder};
use tetra::graphics::DrawParams;
use tetra::input::{Key, MouseButton};
use tetra::math::Vec2;
use tetra::{graphics, input, Context, ContextBuilder, Event, State, TetraError};

const PI: f32 = std::f32::consts::PI;

const PLAYER_SPEED: f32 = 2.0;
const GOBLIN_SPEED: f32 = PLAYER_SPEED * 4.0;

// TODO allow resize window

struct HelpingCircle {
    mesh: Mesh,
    position: Vec2<f32>,
    radius: f32,
}

impl HelpingCircle {
    fn new(ctx: &mut Context, window: &Window, lake: &Lake) -> tetra::Result<Self> {
        let radius = lake.radius / (GOBLIN_SPEED / PLAYER_SPEED);
        let mesh = Mesh::circle(ctx, ShapeStyle::Stroke(1.0), Vec2::zero(), radius)?;
        let position = window.center();
        Ok(Self {
            mesh,
            radius,
            position,
        })
    }

    fn draw(&self, ctx: &mut Context) {
        let draw_params = DrawParams::new()
            .position(self.position)
            .color(Color::WHITE);
        self.mesh.draw(ctx, draw_params);
    }

    fn on_window_resize(
        &mut self,
        ctx: &mut Context,
        window: &Window,
        lake: &Lake,
    ) -> tetra::Result {
        self.radius = lake.radius / (GOBLIN_SPEED / PLAYER_SPEED);
        self.position = window.center();
        self.mesh = Mesh::circle(ctx, ShapeStyle::Stroke(1.0), Vec2::zero(), self.radius)?;
        Ok(())
    }
}

struct Goblin {
    mesh: Mesh,
    position: Vec2<f32>,
    radius: f32,
}

impl Goblin {
    fn new(ctx: &mut Context, window: &Window, lake: &Lake) -> tetra::Result<Self> {
        let radius = 5.0;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        let center = Vec2::new(window.width / 2.0, window.center().y - lake.radius);
        Ok(Self {
            mesh,
            radius,
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
    radius: f32,
}

impl Player {
    fn new(ctx: &mut Context, window: &Window) -> tetra::Result<Self> {
        let radius = 5.0;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        let center = Vec2::new(window.width / 2.0, window.height / 2.0);
        Ok(Self {
            mesh,
            radius,
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
    position: Vec2<f32>,
    radius: f32,
}

impl Lake {
    fn new(ctx: &mut Context, window: &Window) -> tetra::Result<Self> {
        let radius = window.height / 2.0 * 0.8;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        Ok(Self {
            mesh,
            position: window.center(),
            radius,
        })
    }

    fn draw(&self, ctx: &mut Context) {
        let draw_params = DrawParams::new()
            .position(self.position)
            .color(Color::rgb8(0, 0, 255));
        self.mesh.draw(ctx, draw_params);
    }

    fn on_window_resize(&mut self, ctx: &mut Context, window: &Window) -> tetra::Result {
        self.radius = window.height / 2.0 * 0.8;
        self.position = window.center();
        self.mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), self.radius)?;
        Ok(())
    }
}

enum GameResult {
    Playing,
    Ended { text: Text, background_color: Color },
}

struct Window {
    width: f32,
    height: f32,
}

impl Window {
    fn center(&self) -> Vec2<f32> {
        Vec2::new(self.width / 2.0, self.height / 2.0)
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            width: 1280.,
            height: 720.,
        }
    }
}

struct GameState {
    window: Window,
    result: GameResult,
    lake: Lake,
    player: Player,
    goblin: Goblin,
    helping_circle: HelpingCircle,
    player_wins_text: Text,
    goblin_wins_text: Text,
}

impl GameState {
    fn new(ctx: &mut Context, window: Window) -> tetra::Result<Self> {
        let font_builder = VectorFontBuilder::new("./fonts/NewTegomin-Regular.ttf")?;
        let font = font_builder.with_size(ctx, 64.0)?;
        let player_wins_text = Text::new("You win! Congrats!", font.clone());
        let goblin_wins_text = Text::new("Goblin wins!", font.clone());
        let lake = Lake::new(ctx, &window)?;
        let goblin = Goblin::new(ctx, &window, &lake)?;
        let player = Player::new(ctx, &window)?;
        let helping_circle = HelpingCircle::new(ctx, &window, &lake)?;
        Ok(Self {
            window,
            result: GameResult::Playing,
            lake,
            player,
            goblin,
            helping_circle,
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

        let center = self.window.center();

        // 1. find arc length
        // 2. if length < GOBLIN_SPEED, move goblin to player
        // 3. else move goblin to GOBLIN_SPEED point
        let goblin_vector = self.goblin.position - center;
        let player_vector = self.player.position - center;
        let angle = goblin_vector.angle_between(player_vector); // angle in radians

        // https://tutors.com/math-tutors/geometry-help/how-to-find-arc-measure-formula
        let arc_length = self.lake.radius * angle;
        if arc_length <= GOBLIN_SPEED {
            self.goblin.position =
                center + player_vector * (self.lake.radius / player_vector.magnitude());
        } else {
            let arc_length = GOBLIN_SPEED;
            let angle = arc_length / self.lake.radius;
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
            self.goblin.position = center + goblin_rotated;
        }

        let vector_between = self.player.position - self.goblin.position;
        if vector_between.magnitude() < self.player.radius + self.goblin.radius {
            self.result = GameResult::Ended {
                text: self.goblin_wins_text.clone(),
                background_color: Color::rgb8(240, 30, 30),
            };
        } else if (self.player.position.x - center.x).powi(2)
            + (self.player.position.y - center.y).powi(2)
            > self.lake.radius.powi(2)
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
                self.helping_circle.draw(ctx);
                self.player.draw(ctx);
                self.goblin.draw(ctx);
            }
            GameResult::Ended {
                text,
                background_color,
            } => {
                graphics::clear(ctx, *background_color);
                let position = match text.get_bounds(ctx) {
                    Some(rect) => Vec2::new(
                        (self.window.width - rect.width) / 2.,
                        (self.window.height - rect.height) / 2.,
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

    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
        if let Event::Resized { width, height } = event {
            // TODO resize method on all game objects
            self.window.width = width as f32;
            self.window.height = height as f32;
            self.lake.on_window_resize(ctx, &self.window)?;
            self.helping_circle
                .on_window_resize(ctx, &self.window, &self.lake)?;
            // println!("{} {}", width, height);
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    let window = Window::default();
    ContextBuilder::new(
        "Escape the Goblin!",
        window.width as i32,
        window.height as i32,
    )
    .title("Escape the Goblin!")
    .show_mouse(true)
    // .resizable(true)
    .build()?
    .run(|ctx| GameState::new(ctx, window))
}
