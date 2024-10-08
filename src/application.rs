extern crate sdl2;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;
use std::time::{Duration, Instant};

use crate::components::{ComponentBehaviour, Rigidbody};
use crate::gameobject::GameObject;
use crate::input::{Input, InputInfo};
use crate::math::Vec2;
use crate::renderer::{Shader, ShaderProgram};
use crate::time::Timer;
use crate::world::WorldState;
use debug_print::debug_println;
use glam::Mat4;
use ruwren::{
    create_module, BasicFileLoader, Class, FunctionHandle, FunctionSignature, Handle, ModuleLibrary, VMConfig, VMWrapper, VM
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::video::{FullscreenType, GLContext, GLProfile, SwapInterval, Window};
use sdl2::{AudioSubsystem, EventPump, Sdl};

lazy_mut! {
    pub static mut DEBUG_PROGRAM: Option<ShaderProgram> = None;
    pub static mut LINES: Vec<(Vec2, Vec2, crate::renderer::Color)> = Vec::new();
}

#[macro_export]
macro_rules! embed_script {
    ($path: expr, $scripting:ident) => {
        let module_str = $path.to_string();
        let module_name_full = module_str.split("/").collect::<Vec<&str>>();
        let mut module_name = module_name_full[module_name_full.len() - 1];
        let module_name_final = module_name.split(".").collect::<Vec<&str>>();
        $scripting.load_script(module_name_final[0], include_str!($path));
    };
}
pub use embed_script;

#[macro_export]
macro_rules! LilahTypeError {
    ($class: ty, $arg: literal, $t: ty) => {
        eprintln!(
            "--> {} ({}:{})\n |\tArg ({}) must be of type {}",
            stringify!($class),
            file!(),
            line!(),
            $arg,
            stringify!($t)
        )
    };
}
pub use LilahTypeError;

#[macro_export]
macro_rules! LilahTypePanic {
    ($class: ty, $arg: literal, $t: ty) => {
        panic!(
            "--> {} ({}:{})\n |\tArg ({}) must be of type {}",
            stringify!($class),
            file!(),
            line!(),
            $arg,
            stringify!($t)
        );
    };
}
pub use LilahTypePanic;

#[macro_export]
macro_rules! LilahError {
    ($class: ty, $arg: ident) => {
        eprintln!(
            "--> {} ({}:{})\n |\t{}",
            stringify!($class),
            file!(),
            line!(),
            $arg
        )
    };
    ($class: ty, $arg: literal) => {
        eprintln!(
            "--> {} ({}:{})\n |\t{}",
            stringify!($class),
            file!(),
            line!(),
            $arg
        );
    };
    ($class: ty, $arg: expr) => {
        eprintln!(
            "--> {} ({}:{})\n |\t{}",
            stringify!($class),
            file!(),
            line!(),
            $arg
        );
    };
}
pub use LilahError;

#[macro_export]
macro_rules! LilahPanic {
    ($class: ty, $arg: literal) => {
        panic!(
            "--> {} ({}:{})\n |\t{}",
            stringify!($class),
            file!(),
            line!(),
            $arg
        )
    };
    ($class: ty, $arg: ident) => {
        panic!(
            "--> {} ({}:{})\n |\t{}",
            stringify!($class),
            file!(),
            line!(),
            $arg
        )
    };
    ($class: ty, $arg: expr) => {
        panic!(
            "--> {} ({}:{})\n |\t{}",
            stringify!($class),
            file!(),
            line!(),
            $arg
        )
    };
}
pub use LilahPanic;

#[macro_export]
macro_rules! LilahNotFoundError {
    ($class: ty, $t: ty, $arg: ident) => {
        eprintln!(
            "--> {} ({}:{})\n |\tCould Not Find {}({})",
            stringify!($class),
            file!(),
            line!(),
            stringify!($t),
            $arg
        );
    };
}
pub use LilahNotFoundError;

/// Scripting wrapper
pub struct Scripting {
    /// wren VM
    pub vm: VMWrapper,
    /// loaded modules. loaded as Module Name(String), Source(String).
    pub modules: HashMap<String, String>,
}

impl Scripting {
    pub fn new() -> Self {
        let mut modules = HashMap::new();
        //let loader = BasicFileLoader::new().base_dir("");

        let mut lib = ModuleLibrary::new();
        
        io::publish_module(&mut lib);
        crate::math::publish_modules(&mut lib);
        crate::components::publish_modules(&mut lib);
        

        let vm = VMConfig::new()
            .enable_relative_import(true)
            .library(&lib)
            //.script_loader(loader)
            .build();

        //std engine modules
        vm.interpret("io", include_str!("scripts/io.wren")).expect("Failed");
        vm.interpret("math", include_str!("scripts/math.wren")).expect("Failed");
        vm.interpret("app", include_str!("scripts/app.wren")).expect("Failed");
        vm.interpret("game", include_str!("scripts/game.wren")).expect("Failed");
        vm.interpret("ParticleSystem", include_str!("scripts/ParticleSystem.wren")).expect("Failed");
        vm.interpret("Trail", include_str!("scripts/Trail.wren")).expect("Failed");
        modules.insert("ParticleSystem".to_string(), "".to_string());
        modules.insert("Trail".to_string(), "".to_string());

        Self {
            vm: vm,
            modules: modules,
        }
    }

    pub fn load_script(&mut self, module: &str, source: &str) {
        let mod_name = module.to_string();
        let modded_source = source.split("Behaviour {").collect::<Vec<&str>>();
        let static_helpers = 
        format!("\n\tstatic gameobject {{ __gameobject }}\n\tstatic gameobject=(v) {{ __gameobject = GameObjectRef.new(v) }}\n\tstatic gamebehaviour {{ gameobject.behaviourData({}, __uuid) }}\n\tstatic gamebehaviour=(v) {{__uuid = v}}\n\tconstruct new(g) {{ super(g, {}) }}", mod_name, mod_name);
    
        let src = format!(
            "{}Behaviour {{ {} {}\nvar {} = {}.new()",
            modded_source[0],
            static_helpers,
            modded_source[1],
            mod_name.to_lowercase(),
            mod_name
        );

        self.modules.insert(mod_name.clone(), src.clone());

        match self.vm.interpret(mod_name.clone(), src) {
            Ok(_) => {
                debug_println!("Script Loaded: Module->{}", mod_name)
            }
            Err(e) => {
                panic!("Script Error: Could not load Module->{}\n{}", mod_name, e)
            }
        }
    }

    pub fn tick(&mut self, app: &mut App, state: &mut WorldState) {
        let state_class = Scripting::get_class_handle(&self.vm, "app", "Lilah");

        let frame_getter = Scripting::get_getter_handle(&self.vm, "frame");
        let frame_setter = Scripting::get_setter_handle(&self.vm, "frame");

        for m in &self.modules {
            let class = Scripting::get_class_handle(&self.vm, &m.0, &m.0.to_lowercase());
            let obj = Scripting::get_class_handle(&self.vm, &m.0, &m.0);

            Scripting::call_handle(&self.vm, &class, &frame_getter);

            let mut frame = 0;
            self.vm.execute(|vm| {
                if let Some(s) = vm.get_slot_double(0) {
                    frame = s as i32;
                }
            });

            match frame {
                0 => {
                    Scripting::call_fn(&self.vm, &class, "setup", 0);

                    self.vm.execute(|vm| {
                        vm.set_slot_double(1, (frame + 1).into());
                    });

                    Scripting::call_handle(&self.vm, &class, &frame_setter);
                }
                1 => {
                    Scripting::call_fn(&self.vm, &class, "start", 0);

                    self.vm.execute(|vm| {
                        vm.set_slot_double(1, (frame + 1).into());
                    });

                    Scripting::call_handle(&self.vm, &class, &frame_setter);

                    for g in &mut state.gameobjects {
                        let behaviours = g.1.wrap_all::<ComponentBehaviour>();
                        for b in behaviours {
                            if b.get_component() == m.0  {
                                if g.1.init && !g.1.start {
                                    self.vm.execute(|vm| {
                                        vm.set_slot_string(1, g.0.clone());
                                    });
                                    Scripting::call_setter(&self.vm, &obj, "gameobject");
                                    
                                    self.vm.execute(|vm| {
                                        vm.set_slot_string(1, b.uuid.as_str());
                                    });
                                    Scripting::call_setter(&self.vm, &obj, "gamebehaviour");

                                    Scripting::call_fn(&self.vm, &obj, "start", 0);
                                }
                            }
                        }
                    }
                }
                _ => {
                    Scripting::call_fn(&self.vm, &state_class, "tick_fibers", 0);
                    Scripting::call_fn(&self.vm, &class, "update", 0);

                    self.vm.execute(|vm| {
                        vm.set_slot_double(1, (frame + 1).into());
                    });

                    Scripting::call_handle(&self.vm, &class, &frame_setter);
                    
                    for g in &mut state.gameobjects {
                        if !g.1.has_behaviour {
                            continue;
                        }

                        let behaviours = g.1.wrap_all::<ComponentBehaviour>();
                        for b in behaviours {
                            if b.get_component() == m.0 {
                                if g.1.init && !g.1.start {
                                    self.vm.execute(|vm| {
                                        vm.set_slot_string(1, g.0.clone());
                                    });
                                    Scripting::call_setter(&self.vm, &obj, "gameobject");
                                    
                                    self.vm.execute(|vm| {
                                        vm.set_slot_string(1, b.uuid.as_str());
                                    });
                                    Scripting::call_setter(&self.vm, &obj, "gamebehaviour");

                                    Scripting::call_fn(&self.vm, &obj, "start", 0);
                                } else if g.1.start {
                                    self.vm
                                        .execute(|vm| vm.set_slot_string(1, g.0.clone()));
                                    Scripting::call_setter(&self.vm, &obj, "gameobject");

                                    self.vm.execute(|vm| {
                                        vm.set_slot_string(1, b.uuid.as_str());
                                    });
                                    Scripting::call_setter(&self.vm, &obj, "gamebehaviour");

                                    Scripting::call_fn(&self.vm, &obj, "update", 0);

                                    if let Some(body) = g.1.wrap_component::<Rigidbody>() {
                                        if let Some(coll) = &body.colliding {
                                            self.vm.execute(|vm| {
                                                vm.set_slot_new_map(1);
                                                vm.set_slot_string(2, "id");
                                                vm.set_slot_string(3, coll.name.clone());
                                                vm.set_map_value(1, 2,3);
                                                vm.set_slot_string(2, "uuid");
                                                vm.set_slot_string(3, coll.uuid.clone());
                                                vm.set_map_value(1,2, 3); 
                                            });
                                            Scripting::call_fn(&self.vm, &obj, "onCollision", 1);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } 
        }

        self.receive_audio(app, state);
        self.handle_timer(app, state);
        self.receive_state(app, state);
    }

    pub fn get_class_handle<'a>(vm: &'a VMWrapper, module: &str, class: &str) -> Rc<Handle<'a>> {
        vm.execute(|vm| {
            vm.get_variable(module, class, 0);
        });
        vm.get_slot_handle(0)
    }

    pub fn call_getter<'a>(vm: &'a VMWrapper, class: &Rc<Handle<'a>>, function: &str) {
        vm.set_slot_handle(0, &class);
        if let Err(e) = vm.call(FunctionSignature::new_getter(function)) {
            LilahError!(Scripting, e);
        }
    }

    pub fn call_setter<'a>(vm: &'a VMWrapper, class: &Rc<Handle<'a>>, function: &str) {
        vm.set_slot_handle(0, &class);
        if let Err(e) = vm.call(FunctionSignature::new_setter(function)) {
            LilahError!(Scripting, e);
        }
    }

    pub fn call_fn<'a>(vm: &'a VMWrapper, class: &Rc<Handle<'a>>, function: &str, arity: usize) {
        vm.set_slot_handle(0, &class);
        if let Err(e) = vm.call(FunctionSignature::new_function(function, arity)) {
            LilahError!(Scripting, e);
        }
    }

    pub fn call_fn_error_silent<'a>(vm: &'a VMWrapper, class: &Rc<Handle<'a>>, function: &str, arity: usize) {
        vm.set_slot_handle(0, &class);
        if let Err(_) = vm.call(FunctionSignature::new_function(function, arity)) {
            return
        }
    }

    pub fn call_handle<'a>(
        vm: &'a VMWrapper,
        class: &Rc<Handle<'a>>,
        function: &Rc<FunctionHandle<'a>>,
    ) {
        vm.set_slot_handle(0, &class);
        if let Err(e) = vm.call_handle(&function) {
            LilahError!(Scripting, e);
        }
    }

    pub fn get_getter_handle<'a>(vm: &'a VMWrapper, function: &str) -> Rc<FunctionHandle<'a>> {
        vm.make_call_handle(FunctionSignature::new_getter(function))
    }

    pub fn get_setter_handle<'a>(vm: &'a VMWrapper, function: &str) -> Rc<FunctionHandle<'a>> {
        vm.make_call_handle(FunctionSignature::new_setter(function))
    }

    pub fn get_fn_handle<'a>(
        vm: &'a VMWrapper,
        function: &str,
        arity: usize,
    ) -> Rc<FunctionHandle<'a>> {
        vm.make_call_handle(FunctionSignature::new_function(function, arity))
    }
}

/// App wrapper
pub struct App {
    pub gl_context: GLContext,
    pub input: Input,
    pub time: Timer,
    pub default_program: ShaderProgram,
    pub text_program: ShaderProgram,
    pub sort_dirty: bool,
    event_pump: EventPump,
    _audio_context: AudioSubsystem,
    window: Window,
}

impl App {
    pub const DEFAULT_VERT: &'static str = r#"
    #version 330
    out vec2 texCoord;

    in vec2 position;
    in vec2 vertexTexCoord;

    uniform mat4 mvp;

    void main() {
        gl_Position = mvp * vec4(position, 0.0, 1.0);
        texCoord = vertexTexCoord;
    }
    "#;

    pub const DEFAULT_FRAG: &'static str = r#"
    #version 330
    out vec4 FragColor;

    in vec2 texCoord;

    uniform sampler2D texture0;
    uniform vec4 tint;

    void main() {
        vec4 tex = texture(texture0, texCoord) * tint;
        FragColor = vec4(tex.x, tex.y, tex.z, min(tex.w, tint.w));
    }
    "#;

    pub const TEXT_FRAG: &'static str = r#"
    #version 330
    out vec4 FragColor;

    in vec2 texCoord;

    uniform sampler2D texture0;
    uniform vec4 tint;

    void main() {
        vec4 tex = texture(texture0, texCoord);
        FragColor = vec4(tint.x, tint.y, tint.z, min(tex.a, tint.w));
    }
    "#;

    pub const DEBUG_VERT: &'static str = r#"
    #version 330

    in vec2 opacity;
    out vec2 o;
    in vec2 position;
    uniform mat4 mvp;

    void main() {
        o = opacity;
        gl_Position = mvp * vec4(position, 0.0, 1.0);
    }
    "#;

    pub const DEBUG_FRAG: &'static str = r#"
    #version 330 core

    in vec2 o;
    out vec4 FragColor;
    uniform vec4 tint;

    void main() {
        FragColor = vec4(tint.rgb, o.x*tint.a);
    } 
    "#;

    pub fn new(window_title: &str, window_size: Vec2) -> Self {
        let sdl_ctx: Sdl = sdl2::init().unwrap();
        let audio_context = sdl_ctx.audio().unwrap();

        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
        let _mixer_context = sdl2::mixer::init(
            sdl2::mixer::InitFlag::MP3 | sdl2::mixer::InitFlag::FLAC | sdl2::mixer::InitFlag::OGG,
        )
        .unwrap();
        sdl2::mixer::allocate_channels(128);

        let video_subsystem = sdl_ctx.video().unwrap();
        video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync);
        let win: Window = video_subsystem
            .window(window_title, window_size.x as u32, window_size.y as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let gl_ctx = win.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        let event_pump = sdl_ctx.event_pump().unwrap();

        let program = unsafe {
            let vs = Shader::new(App::DEFAULT_VERT, gl::VERTEX_SHADER).unwrap();
            let fs = Shader::new(App::DEFAULT_FRAG, gl::FRAGMENT_SHADER).unwrap();
            ShaderProgram::new(&[fs, vs]).unwrap()
        };

        let text_program = unsafe {
            let vs = Shader::new(App::DEFAULT_VERT, gl::VERTEX_SHADER).unwrap();
            let fs = Shader::new(App::TEXT_FRAG, gl::FRAGMENT_SHADER).unwrap();
            ShaderProgram::new(&[fs, vs]).unwrap()
        };

        unsafe {
            let debug_program = {
                let vs = Shader::new(App::DEBUG_VERT, gl::VERTEX_SHADER).unwrap();
                let fs = Shader::new(App::DEBUG_FRAG, gl::FRAGMENT_SHADER).unwrap();
                ShaderProgram::new(&[fs, vs]).unwrap()
            };

            *crate::application::DEBUG_PROGRAM = Some(debug_program);
        }

        unsafe {
            gl::LineWidth(2.0);
            gl::Enable(gl::BLEND);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
            gl::LineWidth(5.0);
          
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            let aspect = (window_size.x/window_size.y) as f32;

            *crate::math::PROJECTION_MATRIX = Mat4::orthographic_rh_gl(
                -window_size.x as f32/2.0,
                window_size.x as f32/2.0,
                -window_size.y as f32/2.0,
                window_size.y as f32/2.0,
                1000.0,
                -1000.0,
            );

            *crate::application::LINES = Vec::new();
        }

        Self {
            gl_context: gl_ctx,
            window: win,
            event_pump,
            input: Input::new(),
            time: Timer::new(),
            sort_dirty: true,
            _audio_context: audio_context,
            default_program: program,
            text_program: text_program
        }
    }

    pub fn get_window_size(&self) -> Vec2 {
        Vec2::new(self.window.size().0 as f64, self.window.size().1 as f64)
    }

    pub fn toggle_fullscreen(&mut self) {
        match self.window.fullscreen_state() {
            FullscreenType::Off => {
                if let Err(e) = self.window.set_fullscreen(FullscreenType::True) {
                    LilahPanic!(App, e);
                }
            }
            FullscreenType::True => {
                if let Err(e) = self.window.set_fullscreen(FullscreenType::Off) {
                    LilahPanic!(App, e);
                }
            }
            FullscreenType::Desktop => {
                if let Err(e) = self.window.set_fullscreen(FullscreenType::Off) {
                    LilahPanic!(App, e);
                }
            }
        }
    }

    pub fn get_fullscreen(&self) -> bool {
        match self.window.fullscreen_state() {
            FullscreenType::Off => false,
            FullscreenType::True => true,
            FullscreenType::Desktop => true,
        }
    }

    pub fn set_fullscreen(&mut self, set: bool) {
        match set {
            true => {
                if let Err(e) = self.window.set_fullscreen(FullscreenType::Desktop) {
                    LilahPanic!(App, e);
                }
            }
            false => {
                if let Err(e) = self.window.set_fullscreen(FullscreenType::Off) {
                    LilahPanic!(App, e);
                }
            }
        }
    }

    pub fn delta_time(&self) -> f64 {
        self.time.delta_time_scaled()
    }

    pub fn smooth_delta_time(&self) -> f64 {
        self.time.smooth_delta_time
    }

    pub fn handle_input(&mut self) -> bool {
        self.input.update_mouse_pos(Vec2::new(
            self.event_pump.mouse_state().x() as f64,
            self.event_pump.mouse_state().y() as f64,
        ));

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => {
                    return true;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.input.update_mouse((
                        &mouse_btn,
                        &InputInfo {
                            pressed: true,
                            pressed_down: true,
                        },
                    ));
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.input.update_mouse((
                        &mouse_btn,
                        &InputInfo {
                            pressed: false,
                            pressed_down: false,
                        },
                    ));
                }
                Event::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => {
                    self.input.update_mapping((
                        &keycode.unwrap(),
                        &InputInfo {
                            pressed: true,
                            pressed_down: true,
                        },
                    ));
                }
                Event::KeyUp {
                    keycode,
                    repeat: false,
                    ..
                } => {
                    self.input.update_mapping((
                        &keycode.unwrap(),
                        &InputInfo {
                            pressed: false,
                            pressed_down: false,
                        },
                    ));
                }
                _ => {}
            }
        }

        false
    }

    pub fn set_draw_color(&mut self, color: Color) {
        //self.canvas.set_draw_color(color);
    }

    /// Performs time update, canvas clear, and handles input.
    pub fn pre_frame(&mut self) -> bool {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.handle_input()
    }

    /// Draws canvas and sleeps until next frame
    pub fn present_frame(&mut self) {
        self.window.gl_swap_window();
    }
}

impl Scripting {
    pub fn handle_input(&self, app: &mut App, _state: &mut WorldState) {
        let class = Scripting::get_class_handle(&self.vm, "app", "Input");

        let update_mapping_handle = Scripting::get_fn_handle(&self.vm, "update_mapping", 3);

        for entry in &app.input.mappings {
            let a = entry.0.to_string();
            let b = entry.1.pressed;
            let c = entry.1.pressed_down;
            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
                vm.set_slot_bool(2, b);
                vm.set_slot_bool(3, c);
            });
            Scripting::call_handle(&self.vm, &class, &update_mapping_handle);
        }

        let update_mouse_mapping_handle =
            Scripting::get_fn_handle(&self.vm, "update_mouse_mapping", 3);

        for entry in &app.input.mouse_mapping {
            let a = match entry.0 {
                MouseButton::Left => "Left",
                MouseButton::Middle => "Middle",
                MouseButton::Right => "Right",
                _ => "Unknown",
            };

            let b = entry.1.pressed;
            let c = entry.1.pressed_down;
            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
                vm.set_slot_bool(2, b);
                vm.set_slot_bool(3, c);
            });
            Scripting::call_handle(&self.vm, &class, &update_mouse_mapping_handle);
        }

        let update_binding_handle = Scripting::get_fn_handle(&self.vm, "update_binding", 3);

        for entry in &app.input.bindings {
            let a = entry.0.to_string();
            let b = entry.1.negative.to_string();
            let c = entry.1.positive.to_string();

            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
                vm.set_slot_string(2, b);
                vm.set_slot_string(3, c);
            });
            Scripting::call_handle(&self.vm, &class, &update_binding_handle);
        }

        self.vm.execute(|vm| {
            let _ = vm.set_slot_new_foreign("math", "Vec2", app.input.mouse_pos, 1);
        });
        Scripting::call_fn(&self.vm, &class, "set_mouse_pos", 1);
    }

    pub fn handle_timer(&self, app: &mut App, _state: &mut WorldState) {
        let state_class = Scripting::get_class_handle(&self.vm, "app", "Lilah");
        let val = app.delta_time() as f64;
        self.vm.execute(|vm| {
            vm.set_slot_double(1, val);
        });
        Scripting::call_setter(&self.vm, &state_class, "delta_time");

        let val = app.time.fps();
        self.vm.execute(|vm| {
            vm.set_slot_double(1, val);
        });
        Scripting::call_setter(&self.vm, &state_class, "fps");

        let val = app.time.time();
            self.vm.execute(|vm| {
            vm.set_slot_double(1, val);
        });
        Scripting::call_setter(&self.vm, &state_class, "time");
    }

    pub fn send_state(&self, app: &mut App, state: &mut WorldState) {
        let class = Scripting::get_class_handle(&self.vm, "app", "Lilah");
        let input_class = Scripting::get_class_handle(&self.vm, "app", "Input");

        self.vm.execute(|vm| {
            let _ = vm.set_slot_new_foreign(
                "math",
                "Vec2",
                Vec2::new(app.window.size().0 as f64, app.window.size().1 as f64),
                1,
            );
        });

        Scripting::call_setter(&self.vm, &class, "screen_size");

        self.vm.execute(|vm| {
            vm.set_slot_new_map(1);

            for i in state.gameobjects.iter().enumerate() {
                i.1.1.clone().send_to_wren(2, vm);
                vm.set_slot_string(3, i.1.0.clone());
                vm.set_map_value(1, 3, 2);
            }
        });
        Scripting::call_setter(&self.vm, &class, "gameobjects");

        self.vm.execute(|vm| {
            vm.set_slot_bool(1, app.get_fullscreen());
        });
        Scripting::call_setter(&self.vm, &class, "fullscreen");

        let is_pressed = Scripting::get_fn_handle(&self.vm, "is_pressed", 1);

        for entry in &mut app.input.mappings {
            let a = entry.0.to_string();
            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
            });
            Scripting::call_handle(&self.vm, &input_class, &is_pressed);

            let mut b = None;
            self.vm.execute(|vm| {
                if let Some(pressed) = vm.get_slot_bool(0) {
                    b = Some(pressed);
                }
            });

            if let Some(pd) = b {
                entry.1.pressed_down = pd;
            }
        }

        let is_mouse_pressed = Scripting::get_fn_handle(&self.vm, "is_mouse_pressed", 1);

        for entry in &mut app.input.mouse_mapping {
            let a = match entry.0 {
                MouseButton::Left => "Left",
                MouseButton::Middle => "Middle",
                MouseButton::Right => "Right",
                _ => "Unknown",
            };
            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
            });
            Scripting::call_handle(&self.vm, &input_class, &is_mouse_pressed);

            let mut b = entry.1.pressed_down;
            self.vm.execute(|vm| {
                if let Some(pressed) = vm.get_slot_bool(0) {
                    b = pressed;
                }
            });
            entry.1.pressed_down = b;
        }
    }

    pub fn receive_audio(&self, _app: &mut App, state: &mut WorldState) {
        let audio_class = Scripting::get_class_handle(&self.vm, "app", "Audio");

        //recieve audio
        Scripting::call_getter(&self.vm, &audio_class, "dirty");

        let mut dirty = false;
        self.vm.execute(|vm| {
            if let Some(d) = vm.get_slot_bool(0) {
                dirty = d;
            }
        });

        if dirty {
            Scripting::call_getter(&self.vm, &audio_class, "command");

            let mut command = String::from("");
            self.vm.execute(|vm| {
                if let Some(cmd) = vm.get_slot_string(0) {
                    command = cmd;
                }
            });

            Scripting::call_getter(&self.vm, &audio_class, "volume");

            self.vm.execute(|vm| {
                if let Some(vol) = vm.get_slot_double(0) {
                    sdl2::mixer::Music::set_volume(vol as i32);
                }
            });

            match command.as_ref() {
                "play" => {
                    sdl2::mixer::Music::resume();
                }
                "pause" => {
                    sdl2::mixer::Music::pause();
                }
                "pause_fade" => {
                    Scripting::call_getter(&self.vm, &audio_class, "fade");

                    let mut fade = 0;
                    self.vm.execute(|vm| {
                        if let Some(f) = vm.get_slot_double(0) {
                            fade = f as i32;
                        }
                    });
                    let _ = sdl2::mixer::Music::fade_out(fade);
                }
                "start" => {
                    Scripting::call_getter(&self.vm, &audio_class, "music");
                    let mut song = String::from("");

                    self.vm.execute(|vm| {
                        if let Some(file) = vm.get_slot_string(0) {
                            song = file;
                        }
                    });

                    if let Some(music) = state.music.get(&song) {
                        let _ = music.play(-1);
                    }
                }
                "start_fade" => {
                    Scripting::call_getter(&self.vm, &audio_class, "music");
                    let mut song = String::from("");

                    self.vm.execute(|vm| {
                        if let Some(file) = vm.get_slot_string(0) {
                            song = file;
                        }
                    });

                    if let Some(music) = state.music.get(&song) {
                        Scripting::call_getter(&self.vm, &audio_class, "fade");
                        let mut fade = 0;

                        self.vm.execute(|vm| {
                            if let Some(f) = vm.get_slot_double(0) {
                                fade = f as i32;
                            }
                        });

                        sdl2::mixer::Music::halt();
                        let _ = music.fade_in(-1, fade);
                    }
                }
                _ => {}
            }
        } else {
            self.vm.execute(|vm| {
                vm.set_slot_double(1, sdl2::mixer::Music::get_volume() as f64);
            });

            Scripting::call_setter(&self.vm, &audio_class, "volume");
        }
        //end
    }

    pub fn receive_state(&self, app: &mut App, state: &mut WorldState) {
        let state_class = Scripting::get_class_handle(&self.vm, "app", "Lilah");
        let ui_class = Scripting::get_class_handle(&self.vm, "app", "UI");

        Scripting::call_getter(&self.vm, &state_class, "gameobjects_values");

        self.vm.execute(|vm| {
            if let Some(count) = vm.get_list_count(0) {
                for i in 0..count {
                    vm.get_list_element(0, i as i32, 1);

                    let go = vm.get_slot_foreign::<GameObject>(1);
                    if let Some(g) = go {
                        state.insert(g);
                    }
                }
            }
        });

        Scripting::call_getter(&self.vm, &state_class, "destroy");

        self.vm.execute(|vm| {
            if let Some(count) = vm.get_list_count(0) {
                if count > 0 {
                    app.sort_dirty = true;
                }

                for i in 0..count {
                    vm.get_list_element(0, i as i32, 1);

                    let go = vm.get_slot_string(1);
                    if let Some(g) = go {
                        state.gameobjects.remove(&g);
                    }
                }
            }
        });

        Scripting::call_getter(&self.vm, &state_class, "fullscreen");

        self.vm.execute(|vm| {
            if let Some(is_sfull) = vm.get_slot_bool(0) {
                if is_sfull != app.get_fullscreen() {
                    app.toggle_fullscreen();
                }
            }
        });

        Scripting::call_fn(&self.vm, &state_class, "clear", 0);

        Scripting::call_fn(&self.vm, &ui_class, "tick", 0);
    }
}

pub struct Fs {}
impl Fs {
    fn wren_read_as_str(vm: &VM) {
        if let Some(file) = vm.get_slot_string(1) {
            let file_in = std::fs::read_to_string(file);

            match file_in {
                Ok(v) => vm.set_slot_string(0, v),
                Err(e) => {
                    LilahError!(Fs, e);
                    vm.set_slot_null(0);
                }
            }
        } else {
            LilahTypeError!(String, 1, Fs);
        }
    }
    fn wren_write_to_str(vm: &VM) {
        if let Some(file) = vm.get_slot_string(1) {
            if let Some(contents) = vm.get_slot_string(2) {
                let file_in = std::fs::write(file, contents);

                match file_in {
                    Ok(_) => {
                        vm.set_slot_bool(0, true);
                    },
                    Err(e) => {
                        LilahError!(Fs, e);
                        vm.set_slot_bool(0, false);
                    }
                }
            } else {
               LilahTypeError!(String, 2, Fs); 
            }
        } else {
            LilahTypeError!(String, 1, Fs);
        }
    }
}

impl Class for Fs {
    fn initialize(_: &VM) -> Self {
        LilahPanic!(Component, "Cannot instantiate static class");
    }
}

create_module! (
    class("Fs") crate::application::Fs => fs {
        static(fn "read", 1) wren_read_as_str,
        static(fn "write", 2) wren_write_to_str
    }

    module => io
);

pub fn publish_modules(lib: &mut ModuleLibrary) {
    io::publish_module(lib);
}


