mod states;
mod models;
mod mapping;

use ggez::{
    event,
    GameResult,
};
use states::ingame::player::Player;
use models::Input;
use crate::states::ingame::InGame;


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("hello_world", "mkd")
        .add_resource_path("./resources");
    let (mut ctx, event_loop) = cb.build()?;
    let state = InGame::new(&mut ctx)?;

    ctx.gfx.set_window_title("seed ggez");
    event::run(ctx, event_loop, state)
}
