use std::env;

use lilah::application::*;
use lilah::math::Vec2;
use lilah::world::*;

fn setup(app: &mut App, state: &mut WorldState, scripting: &mut Scripting) {
    embed_script!("assets/scripts/CamFollow.wren", scripting);
    embed_script!("assets/scripts/Player.wren", scripting);
    embed_scene_data!("assets/Untitled.json", state);
    embed_texture!("assets/skullboy.png", state, app);
    embed_texture!("assets/tiles.png", state, app);
}

pub fn main() {
    let mut app = App::new("Adventure", Vec2::new(800.0,600.0));
    let mut scripting = Scripting::new();

    World::new()
        .setup(Box::new(setup))
        .run(&mut app, &mut scripting);
}
