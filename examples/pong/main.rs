use lilah::application::*;
use lilah::math::Vec2;
use lilah::world::*;

fn setup(app : &mut App, state : &mut WorldState, scripting : &mut Scripting) {
    embed_script!("assets/scripts/Paddle.wren", scripting);
    embed_script!("assets/scripts/Ball.wren", scripting);
    embed_script!("assets/scripts/Game.wren", scripting);
    embed_texture!("assets/paddle.png", gl::REPEAT, gl::NEAREST, state, app);
    embed_texture!("assets/ball.png", gl::REPEAT, gl::NEAREST, state, app);
    embed_texture!("assets/line.png", gl::REPEAT, gl::NEAREST, state, app);
    embed_font!("assets/Lora-Regular.ttf", state);
}

pub fn main() {  
    let mut app = App::new("Misc", Vec2::new(800.0, 600.0));
    let mut scripting = Scripting::new();

    World::new()
        .setup(Box::new(setup))
    .run(&mut app, &mut scripting);  
}