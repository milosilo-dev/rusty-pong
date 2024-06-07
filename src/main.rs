use ggez::{conf::{self, FullscreenType, NumSamples, WindowMode, WindowSetup}, event, glam::{self}, graphics::{self, DrawParam}, input::keyboard::KeyCode, mint, Context, ContextBuilder, GameError, GameResult};
use rand::Rng;

const WIDTH : f32 = 800.0;
const HEIGHT : f32 = 600.0;

pub fn main(){

    let setup = WindowSetup {
        title: "Pong!".to_owned(),    
        samples: NumSamples::One,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let mode = WindowMode {
        width: WIDTH,
        height: HEIGHT,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None, 
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("Pong Dream", "person")
        .default_conf(c)
        .window_setup(setup)
        .window_mode(mode)
        .build()
        .unwrap();

    let state = State {
        dt: std::time::Duration::new(0, 0),
        last_second: 0.0,
        timescale : 1.0,
        ball : Ball { x: WIDTH / 2.0, y: HEIGHT / 2.0, velosity_x: 1.0, velosity_y: (rand::thread_rng().gen_range(0..100) as f32 / 100.0)},
        paddle1: graphics::Rect::new(10.0, (HEIGHT / 2.0) - 50.0,  25.0, 100.0),
        paddle2: graphics::Rect::new(WIDTH - 35.0, (HEIGHT / 2.0) - 50.0, 25.0, 100.0),
        p1_score: 0,
        p2_score: 0,
        p1_text: graphics::Text::new("0"),
        p2_text: graphics::Text::new("0"),
    };

    event::run(ctx, event_loop, state);
}

fn collide(rect: graphics::Rect, x: f32, y: f32) -> bool{
    if (rect.x < x) & (rect.y < y){
        if (rect.x + rect.w > x) & (rect.y + rect.h > y){
            return true;
        }else{
            return false;
        }
    }else{
        return false;
    }
}

struct Ball {
    x: f32,
    y: f32,
    velosity_x: f32,
    velosity_y: f32,
}

struct State {
    dt: std::time::Duration,
    last_second: f32,
    timescale: f32,
    ball: Ball,
    paddle1: graphics::Rect,
    paddle2: graphics::Rect,
    p1_score: i32,
    p2_score: i32,
    p1_text: graphics::Text,
    p2_text: graphics::Text,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        self.timescale += self.dt.as_millis() as f32 * 0.00001;
        if ctx.time.ticks() as f32 > self.last_second + 100.0{
            print!("{}", self.timescale.to_string());
            self.last_second = ctx.time.ticks() as f32;
        }

        let k_ctx = &ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::W) {
            self.paddle1.y -= 7.0 * self.timescale;
        }
        else if k_ctx.is_key_pressed(KeyCode::S) {
            self.paddle1.y += 7.0 * self.timescale;
        }

        if k_ctx.is_key_pressed(KeyCode::Up) {
            self.paddle2.y -= 7.0 * self.timescale;
        }
        else if k_ctx.is_key_pressed(KeyCode::Down) {
            self.paddle2.y += 7.0 * self.timescale;
        }

        self.ball.x += self.ball.velosity_x * self.dt.as_millis() as f32 * 0.2 * self.timescale;
        self.ball.y += self.ball.velosity_y * self.dt.as_millis() as f32 * 0.2 * self.timescale;
        
        if collide(self.paddle1, self.ball.x, self.ball.y){
            let dist : f32 = self.paddle1.y + (self.paddle1.h / 2.0) - self.ball.y;

            self.ball.velosity_x = 0.0 - self.ball.velosity_x  * self.timescale;
            self.ball.velosity_y = -dist * 0.03  * self.timescale;
        }

        if collide(self.paddle2, self.ball.x, self.ball.y){
            let dist : f32 = self.paddle2.y + (self.paddle2.h / 2.0) - self.ball.y;

            self.ball.velosity_x = 0.0 - self.ball.velosity_x  * self.timescale;
            self.ball.velosity_y = -dist * 0.03 * self.timescale;
        }

        if self.ball.y > HEIGHT || self.ball.y < 0.0{
            self.ball.velosity_y = 0.0 - self.ball.velosity_y * self.timescale;
        }

        if self.ball.x > WIDTH {
            self.p1_score += 1;
            self.p1_text = graphics::Text::new(self.p1_score.to_string());
            self.ball = Ball { x: WIDTH / 2.0, y: HEIGHT / 2.0, velosity_x: -1.0, velosity_y: (rand::thread_rng().gen_range(0..100) as f32 / 100.0)};
            self.paddle1 = graphics::Rect::new(10.0, (HEIGHT / 2.0) - 50.0,  25.0, 100.0);
            self.paddle2 = graphics::Rect::new(WIDTH - 35.0, (HEIGHT / 2.0) - 50.0, 25.0, 100.0);
            self.timescale = 1.0;
        }

        if self.ball.x < 0.0 {
            self.p2_score += 1;
            self.p2_text = graphics::Text::new(self.p2_score.to_string());
            self.ball = Ball { x: WIDTH / 2.0, y: HEIGHT / 2.0, velosity_x: 1.0, velosity_y: (rand::thread_rng().gen_range(0..100) as f32 / 100.0)};
            self.paddle1 = graphics::Rect::new(10.0, (HEIGHT / 2.0) - 50.0,  25.0, 100.0);
            self.paddle2 = graphics::Rect::new(WIDTH - 35.0, (HEIGHT / 2.0) - 50.0, 25.0, 100.0);
            self.timescale = 1.0;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let ball = graphics::Mesh::new_circle(
            ctx, 
            graphics::DrawMode::fill(), 
            mint::Point2{x: self.ball.x, y: self.ball.y}, 
            10.0, 
            0.1, 
            graphics::Color { r: (255.0), g: (255.0), b: (255.0), a: (255.0) }
        )?;
        canvas.draw(&ball, graphics::DrawParam::default());

        let paddle1 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            self.paddle1, 
            graphics::Color::WHITE
        )?;
        canvas.draw(&paddle1, graphics::DrawParam::default());

        let paddle2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            self.paddle2, 
            graphics::Color::WHITE
        )?;
        canvas.draw(&paddle2, graphics::DrawParam::default());

        let dest1 = glam::vec2(0.0, 0.0);
        canvas.draw(&self.p1_text, DrawParam::default().dest(dest1));

        let dest2 = glam::vec2(WIDTH - 20.0, 20.0);
        canvas.draw(&self.p2_text, DrawParam::default().dest(dest2));

        canvas.finish(ctx)?;
        Ok(())
    }
}