use lilah::application::App;
use lilah::application::Scripting;
use lilah::load_script;
use lilah::world::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::rwops;
use sdl2::rwops::RWops;

fn setup(app : &mut App, state : &mut WorldState, scripting : &mut Scripting) {
    load_script!("assets/scripts/Player.wren", scripting);
    load_script!("assets/scripts/Player2.wren", scripting);

    load_texture!("assets/test.png", state, app);
    state.fonts.insert("assets/Lora-Regular.ttf".to_string(), include_bytes!("assets/Lora-Regular.ttf").to_vec());
}

pub fn main() {  
    let mut app = App::new("SDL");
    let mut scripting = Scripting::new();

    World::new()
        .setup(Box::new(setup))
    .run(&mut app, &mut scripting);  
}