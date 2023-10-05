use lilah::application::*;
use lilah::world::*;

fn setup(app : &mut App, state : &mut WorldState, scripting : &mut Scripting) {
    embed_script!("assets/scripts/Player.wren", scripting);
    embed_script!("assets/scripts/Player2.wren", scripting);
    embed_texture!("assets/test.png", state, app);
    embed_font!("assets/Lora-Regular.ttf", state);
    embed_music!("assets/test.mp3", state);
}

pub fn main() {  
    let mut app = App::new("SDL");
    let mut scripting = Scripting::new();

    World::new()
        .setup(Box::new(setup))
    .run(&mut app, &mut scripting);  
}