use graphics::Color;
use lazy_static::lazy_static;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Text, VectorFontBuilder};
use tetra::graphics::DrawParams;
use tetra::input::{Key, MouseButton};
use tetra::math::Vec2;
use tetra::{graphics, input, Context, ContextBuilder, Event, State, TetraError};

const PI: f32 = std::f32::consts::PI;

// TODO add starting screen?
// TODO refactor code

// goblin is faster than player
const SPEED_RATIO: f32 = 4.0;

const MINIMAL_WINDOW_WIDTH: i32 = 640;
const MINIMAL_WINDOW_HEIGHT: i32 = 640;

// Все элементы с черной (000000) обводкой в 1px

lazy_static! {
    static ref COLOR_GOBLIN: Color = Color::hex("A71B1B");
    static ref COLOR_PINE: Color = Color::hex("475725");
    static ref COLOR_BACKGROUND: Gradient = Gradient {
        from: Color::hex("5F703B"),
        to: Color::hex("888D4F")
    };
    static ref COLOR_LAKE: Gradient = Gradient {
        from: Color::hex("415781"),
        to: Color::hex("2C3E61")
    };
}

struct Gradient {
    from: Color,
    to: Color,
}

struct HelpingCircle {
    mesh: Mesh,
    position: Vec2<f32>,
    radius: f32,
}

impl HelpingCircle {
    fn new(ctx: &mut Context, window: &Window, lake: &Lake) -> tetra::Result<Self> {
        let radius = lake.radius / SPEED_RATIO;
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
        self.radius = lake.radius / SPEED_RATIO;
        self.position = window.center();
        self.mesh = Mesh::circle(ctx, ShapeStyle::Stroke(1.0), Vec2::zero(), self.radius)?;
        Ok(())
    }
}

struct Goblin {
    mesh: Mesh,
    position: Vec2<f32>,
    radius: f32,
    speed: f32,
}

impl Goblin {
    fn new(
        ctx: &mut Context,
        window: &Window,
        lake: &Lake,
        player: &Player,
    ) -> tetra::Result<Self> {
        let radius = 5.0;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        let center = Vec2::new(window.width / 2.0, window.center().y - lake.radius);
        Ok(Self {
            mesh,
            radius,
            position: center,
            speed: player.speed * SPEED_RATIO,
        })
    }

    fn draw(&self, ctx: &mut Context) {
        let draw_params = DrawParams::new()
            .position(self.position)
            .color(*COLOR_GOBLIN);
        self.mesh.draw(ctx, draw_params);
    }

    fn on_window_resize(
        &mut self,
        ctx: &mut Context,
        window: &Window,
        previous_window: &Window,
    ) -> tetra::Result {
        let ratio = window.height / previous_window.height;
        self.speed *= ratio;
        self.radius *= ratio;
        self.mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), self.radius)?;
        let vector = (self.position - previous_window.center()) * ratio;
        self.position = window.center() + vector;
        Ok(())
    }

    fn reset_to_initial_state(&mut self, window: &Window, lake: &Lake) {
        self.position = Vec2::new(window.width / 2.0, window.center().y - lake.radius);
    }
}

struct Player {
    mesh: Mesh,
    position: Vec2<f32>,
    radius: f32,
    speed: f32,
}

impl Player {
    fn new(ctx: &mut Context, window: &Window) -> tetra::Result<Self> {
        let radius = 5.0;
        let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), radius)?;
        Ok(Self {
            mesh,
            radius,
            position: window.center(),
            speed: 2.0,
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
        previous_window: &Window,
    ) -> tetra::Result {
        let ratio = window.height / previous_window.height;
        self.speed *= ratio;
        self.radius *= ratio;
        self.mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), self.radius)?;
        let vector = self.position - previous_window.center();
        // new position depends on heights ratio because radius of lake depends on height
        self.position = window.center() + vector * ratio;
        Ok(())
    }

    fn reset_to_initial_state(&mut self, window: &Window) {
        self.position = window.center();
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
            .color(COLOR_LAKE.to);
        self.mesh.draw(ctx, draw_params);
    }

    fn on_window_resize(&mut self, ctx: &mut Context, window: &Window) -> tetra::Result {
        self.radius = window.height / 2.0 * 0.8;
        self.position = window.center();
        self.mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), self.radius)?;
        Ok(())
    }
}

#[derive(Clone)]
struct EndScene {
    message: Text,
    auxiliary_message: Text,
    background_color: Color,
}

impl EndScene {
    fn player_wins(ctx: &mut Context) -> tetra::Result<Self> {
        Self::new(ctx, "You win! Congrats!", Color::rgb8(30, 144, 255))
    }

    fn goblin_wins(ctx: &mut Context) -> tetra::Result<Self> {
        Self::new(ctx, "Goblin wins!", Color::rgb8(240, 30, 30))
    }

    fn new(ctx: &mut Context, msg: &str, color: Color) -> tetra::Result<Self> {
        let font_builder = VectorFontBuilder::new("./resources/NewTegomin-Regular.ttf")?;
        let font = font_builder.with_size(ctx, 64.0)?;
        let small_font = font_builder.with_size(ctx, 32.0)?;
        Ok(Self {
            message: Text::new(msg, font),
            auxiliary_message: Text::new("To play again press Space", small_font),
            background_color: color,
        })
    }

    fn draw(&mut self, ctx: &mut Context, window: &Window) {
        graphics::clear(ctx, self.background_color);
        let main_position = Self::text_position(&mut self.message, ctx, window, 0.5);
        let aux_position = Self::text_position(&mut self.auxiliary_message, ctx, window, 0.75);
        let color = Color::WHITE;
        self.message
            .draw(ctx, DrawParams::new().position(main_position).color(color));
        self.auxiliary_message
            .draw(ctx, DrawParams::new().position(aux_position).color(color));
    }

    fn text_position(
        text: &mut Text,
        ctx: &mut Context,
        window: &Window,
        offset: f32,
    ) -> Vec2<f32> {
        match text.get_bounds(ctx) {
            Some(rect) => Vec2::new(
                (window.width - rect.width) / 2.,
                window.height * offset - rect.height / 2.,
            ),
            None => Vec2::zero(),
        }
    }
}

enum GameResult {
    Playing,
    Ended(EndScene),
}

#[derive(Copy, Clone)]
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
        let (width, height) = (640., 480.);
        Self { width, height }
    }
}

struct GameState {
    window: Window,
    result: GameResult,
    lake: Lake,
    player: Player,
    goblin: Goblin,
    helping_circle: HelpingCircle,
    player_wins: EndScene,
    goblin_wins: EndScene,
}

impl GameState {
    fn new(ctx: &mut Context, window: Window) -> tetra::Result<Self> {
        let lake = Lake::new(ctx, &window)?;
        let player = Player::new(ctx, &window)?;
        let goblin = Goblin::new(ctx, &window, &lake, &player)?;
        let helping_circle = HelpingCircle::new(ctx, &window, &lake)?;
        Ok(Self {
            window,
            result: GameResult::Playing,
            lake,
            player,
            goblin,
            helping_circle,
            player_wins: EndScene::player_wins(ctx)?,
            goblin_wins: EndScene::goblin_wins(ctx)?,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        if let GameResult::Ended { .. } = self.result {
            if input::is_key_down(ctx, Key::Space) {
                self.result = GameResult::Playing;
                self.player.reset_to_initial_state(&self.window);
                self.goblin.reset_to_initial_state(&self.window, &self.lake);
            } else {
                return Ok(());
            }
        }

        if input::is_mouse_button_down(ctx, MouseButton::Left) {
            let mouse_position = input::get_mouse_position(ctx);
            let difference = mouse_position - self.player.position;
            let magnitude = difference.magnitude();
            let position = if magnitude > self.player.speed {
                self.player.position + difference * self.player.speed / magnitude
            } else {
                mouse_position
            };
            self.player.position = position
        } else {
            let y = if input::is_key_down(ctx, Key::W) {
                Some(-self.player.speed)
            } else if input::is_key_down(ctx, Key::S) {
                Some(self.player.speed)
            } else {
                None
            };
            let x = if input::is_key_down(ctx, Key::A) {
                Some(-self.player.speed)
            } else if input::is_key_down(ctx, Key::D) {
                Some(self.player.speed)
            } else {
                None
            };
            let player_move = match (x, y) {
                (Some(x), Some(y)) => {
                    let hypotenuse = self.player.speed;
                    let cathetus = (hypotenuse.powi(2) / 2.0).sqrt();
                    Vec2::new(cathetus * x.signum(), cathetus * y.signum())
                }
                (Some(x), None) => Vec2::new(x, 0.0),
                (None, Some(y)) => Vec2::new(0.0, y),
                (None, None) => Vec2::zero(),
            };
            self.player.position += player_move;
        }

        let center = self.window.center();

        // 1. find arc length
        // 2. if length < self.goblin.speed, move goblin to player
        // 3. else move goblin to self.goblin.speed point
        let goblin_vector = self.goblin.position - center;
        let player_vector = self.player.position - center;
        let angle = goblin_vector.angle_between(player_vector); // angle in radians

        // https://tutors.com/math-tutors/geometry-help/how-to-find-arc-measure-formula
        let arc_length = self.lake.radius * angle;
        if arc_length <= self.goblin.speed {
            self.goblin.position =
                center + player_vector * (self.lake.radius / player_vector.magnitude());
        } else {
            let arc_length = self.goblin.speed;
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
            self.result = GameResult::Ended(self.goblin_wins.clone());
        } else if (self.player.position - center).magnitude() > self.lake.radius {
            self.result = GameResult::Ended(self.player_wins.clone());
        };

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        match &mut self.result {
            GameResult::Playing => {
                graphics::clear(ctx, COLOR_BACKGROUND.to);
                self.lake.draw(ctx);
                self.helping_circle.draw(ctx);
                self.player.draw(ctx);
                self.goblin.draw(ctx);
            }
            GameResult::Ended(scene) => {
                scene.draw(ctx, &self.window);
            }
        }

        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
        if let Event::Resized { width, height } = event {
            let previous_window = self.window.clone();
            let (width, height) = if width < MINIMAL_WINDOW_WIDTH {
                tetra::window::set_size(ctx, MINIMAL_WINDOW_WIDTH, height)?;
                (MINIMAL_WINDOW_WIDTH, height)
            } else if height < MINIMAL_WINDOW_HEIGHT {
                tetra::window::set_size(ctx, width, MINIMAL_WINDOW_HEIGHT)?;
                (width, MINIMAL_WINDOW_HEIGHT)
            } else {
                (width, height)
            };
            self.window.width = width as f32;
            self.window.height = height as f32;
            self.lake.on_window_resize(ctx, &self.window)?;
            self.player
                .on_window_resize(ctx, &self.window, &previous_window)?;
            self.goblin
                .on_window_resize(ctx, &self.window, &previous_window)?;
            self.helping_circle
                .on_window_resize(ctx, &self.window, &self.lake)?;
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
    .resizable(true)
    .build()?
    .run(|ctx| GameState::new(ctx, window))
}
