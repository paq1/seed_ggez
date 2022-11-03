use ggez::{event, GameResult, graphics};
use ggez::glam::Vec2;
use crate::{Input, Player};

pub mod player;


pub struct InGame {
    show_debug: bool,
    player: Player,
    player_image: graphics::Image,
    dt: f32
}

impl InGame {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<InGame> {
        let state = InGame {
            show_debug: true,
            player: Player :: new((100.0, 100.0)),
            player_image: graphics::Image::from_path(ctx, "/player.png")?,
            dt: 0.0
        };
        Ok(state)
    }

    pub fn update_kb(&mut self, _ctx: &mut ggez::Context) -> GameResult {

        let k_ctx = &_ctx.keyboard;

        let keys = k_ctx
            .pressed_keys()
            .iter()
            .filter_map(|keycode| Input::from_keycode(keycode))
            .collect::<Vec<Input>>();

        self.player.update_deplacement(keys, self.dt);
        self.update_activate_debug(_ctx);

        Ok(())
    }

    pub fn update_dt(&mut self, ctx: &mut ggez::Context) {
        self.dt = ctx.time.delta().as_secs_f32();
    }

    pub fn update_activate_debug(&mut self, ctx: &mut ggez::Context) {
        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::F1) {
            self.show_debug = !self.show_debug;
        }
    }
}

impl event::EventHandler<ggez::GameError> for InGame {

    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        self.update_dt(_ctx);
        self.update_kb(_ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );
        let text = ggez::graphics::Text::new(format!("fps ({})", ctx.time.fps()));
        let text_dt = ggez::graphics::Text::new(format!("dt ({})", self.dt));

        canvas.draw(&self.player_image, Vec2 { x: self.player.position.0 as f32, y: self.player.position.1 as f32 });
        if self.show_debug {
            canvas.draw(&text, Vec2 { x: 0.0, y: 0.0 });
            canvas.draw(&text_dt, Vec2 { x: 0.0, y: 32.0 });
        }

        canvas.finish(ctx)
    }
}