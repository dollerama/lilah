use std::env;

use lilah::application::*;
use lilah::math::Vec2;
use lilah::world::*;

fn setup(app: &mut App, state: &mut WorldState, scripting: &mut Scripting) {
    embed_script!("assets/scripts/Player.wren", scripting);
    embed_script!("assets/scripts/Player2.wren", scripting);
    embed_scene_data!("assets/Untitled.json", state);
    embed_texture!("assets/test.png", gl::REPEAT, gl::NEAREST, state, app);
    embed_texture!("assets/test2.png", gl::REPEAT, gl::NEAREST, state, app);
    embed_font!("assets/Lora-Regular.ttf", state);
    embed_music!("assets/test.mp3", state);
    embed_sfx!("assets/sfx.wav", state);
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new("Misc", Vec2::new(800.0,600.0));
    let mut scripting = Scripting::new();

    World::new()
        .setup(Box::new(setup))
        .run(&mut app, &mut scripting);
}
