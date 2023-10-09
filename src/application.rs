extern crate sdl2;
use std::collections::HashMap;
use std::time::Duration;

use debug_print::debug_println;
use ruwren::{ModuleLibrary, VMConfig, VMWrapper, BasicFileLoader, FunctionSignature};
use sdl2::keyboard::Keycode;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::mouse::MouseButton;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{Sdl, EventPump, AudioSubsystem};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::video::{Window, WindowContext, FullscreenType};
use sdl2::render::{Canvas, TextureCreator};
use crate::components::ComponentBehaviour;
use crate::gameobject::GameObject;
use crate::input::{Input, InputInfo};
use crate::math::Vec2;
use crate::time::Timer;
use crate::world::WorldState;

#[macro_export]
macro_rules! embed_script {
    ($path: expr, $scripting:ident) => {
        let module_str = $path.to_string();
        let module_name_full = module_str.split("/").collect::<Vec<&str>>();
        let mut module_name = module_name_full[module_name_full.len()-1];
        let module_name_final = module_name.split(".").collect::<Vec<&str>>();
        $scripting.load_script(module_name_final[0], include_str!($path));
    };
}
pub use embed_script;

#[macro_export]
macro_rules! LilahTypeError {
    ($class: ty, $arg: literal, $t: ty) => {
        eprintln!("--> {} ({}:{})\n |\tArg ({}) must be of type {}", stringify!($class), file!(), line!(), $arg, stringify!($t));
    }
}
pub use LilahTypeError;

#[macro_export]
macro_rules! LilahTypePanic {
    ($class: ty, $arg: literal, $t: ty) => {
        panic!("--> {} ({}:{})\n |\tArg ({}) must be of type {}", stringify!($class), file!(), line!(), $arg, stringify!($t));
    }
}
pub use LilahTypePanic;

#[macro_export]
macro_rules! LilahError {
    ($class: ty, $arg: ident) => {
        eprintln!("--> {} ({}:{})\n |\t{}", stringify!($class), file!(), line!(), $arg);
    };
    ($class: ty, $arg: literal) => {
        eprintln!("--> {} ({}:{})\n |\t{}", stringify!($class), file!(), line!(), $arg);
    };
    ($class: ty, $arg: expr) => {
        eprintln!("--> {} ({}:{})\n |\t{}", stringify!($class), file!(), line!(), $arg);
    };
}
pub use LilahError;

#[macro_export]
macro_rules! LilahPanic {
    ($class: ty, $arg: literal) => {
        panic!("--> {} ({}:{})\n |\t{}", stringify!($class), file!(), line!(), $arg)
    };
    ($class: ty, $arg: ident) => {
        panic!("--> {} ({}:{})\n |\t{}", stringify!($class), file!(), line!(), $arg)
    };
    ($class: ty, $arg: expr) => {
        panic!("--> {} ({}:{})\n |\t{}", stringify!($class), file!(), line!(), $arg)
    };
}
pub use LilahPanic;

#[macro_export]
macro_rules! LilahNotFoundError {
    ($class: ty, $t: ty, $arg: ident) => {
        eprintln!("--> {} ({}:{})\n |\tCould Not Find {}({})", stringify!($class), file!(), line!(), stringify!($t), $arg);
    }
}
pub use LilahNotFoundError;

/// Scripting wrapper
pub struct Scripting {
    /// wren VM
    pub vm : VMWrapper,
    /// loaded modules. loaded as Module Name(String), Source(String).
    pub modules : HashMap<String, String>
}

impl Scripting {
    pub fn new() -> Self {
        let modules = HashMap::new();
        let loader = BasicFileLoader::new().base_dir("");

        let mut lib = ModuleLibrary::new();

        crate::math::publish_modules(&mut lib);
        crate::components::publish_modules(&mut lib);

        let vm = VMConfig::new().enable_relative_import(true).library(&lib).script_loader(loader).build();

        //std engine modules
        vm.interpret("math", include_str!("scripts/math.wren")).unwrap();
        vm.interpret("app", include_str!("scripts/app.wren")).unwrap();
        vm.interpret("engine", include_str!("scripts/engine.wren")).unwrap();

        Self {
            vm : vm,
            modules : modules
        }
    }

    pub fn load_script(&mut self, module: &str, source: &str) {
        let mod_name = module.to_string();
        let src = format!("{}\nvar {} = {}.new()", source, mod_name.to_lowercase(), mod_name);

        self.modules.insert(mod_name.clone(), src.clone());

        match self.vm.interpret(format!("{}Main", mod_name), src) {
            Ok(_) => { debug_println!("Script Loaded: Module->{}", mod_name) }
            Err(e) => { panic!("Script Error: Could not load Module->{}\n{}", mod_name, e) }
        }
    }
    
    pub fn tick(&mut self, app : &mut App, state : &mut WorldState) {
        for m in &self.modules {
            self.vm.execute(|vm| {
                vm.ensure_slots(1);
                vm.get_variable(format!("{}Main", m.0), m.0.to_lowercase(), 0);
            });

            let class = self.vm.get_slot_handle(0);

            let frame_getter = self.vm.make_call_handle(FunctionSignature::new_getter("frame"));
            let frame_setter = self.vm.make_call_handle(FunctionSignature::new_setter("frame"));
            
            self.vm.set_slot_handle(0, &class);
            let _ = self.vm.call_handle(&frame_getter);

            let mut frame = 0;
            self.vm.execute(|vm| {
                if let Some(s) = vm.get_slot_double(0) {
                    frame = s as i32;
                }
            });

            match frame {
                0 => {
                    let setup_function = self.vm.make_call_handle(FunctionSignature::new_function("setup", 0));

                    self.vm.set_slot_handle(0, &class);
                    let _ = self.vm.call_handle(&setup_function);

                    self.vm.execute(|vm| {
                        vm.set_slot_double(1, (frame+1).into());
                    });

                    self.vm.set_slot_handle(0, &class);
                    let _ = self.vm.call_handle(&frame_setter);
                }
                1 => {
                    let start_function = self.vm.make_call_handle(FunctionSignature::new_function("start", 0));

                    self.vm.set_slot_handle(0, &class);
                    let _ = self.vm.call_handle(&start_function);

                    self.vm.execute(|vm| {
                        vm.set_slot_double(1, (frame+1).into());
                    });

                    self.vm.set_slot_handle(0, &class);
                    let _ = self.vm.call_handle(&frame_setter);               
                }
                _ => {
                    let update_function = self.vm.make_call_handle(FunctionSignature::new_function("update", 0));

                    self.vm.set_slot_handle(0, &class);
                    let _ = self.vm.call_handle(&update_function);

                    self.vm.execute(|vm| {
                        vm.set_slot_double(1, (frame+1).into());
                    });

                    self.vm.set_slot_handle(0, &class);
                    let _ = self.vm.call_handle(&frame_setter);
                }
            }

            for g in &mut state.gameobjects {
                if g.1.has::<ComponentBehaviour>() {
                    if g.1.get::<ComponentBehaviour>().get_component() == m.0 {
                        self.vm.execute(|vm| {
                            vm.ensure_slots(1);
                            vm.get_variable(format!("{}Main", m.0), m.0, 0);
                        });

                        if g.1.init && !g.1.start {
                            let static_start_function = self.vm.make_call_handle(FunctionSignature::new_function("start", 1));
                            self.vm.execute(|vm| {
                                vm.set_slot_double(1, g.1.wren_id as f64)
                            });
                            let _ = self.vm.call_handle(&static_start_function);
                        }
                        if g.1.init && g.1.start {
                            let static_update_function = self.vm.make_call_handle(FunctionSignature::new_function("update", 1));
                            self.vm.execute(|vm| {
                                vm.set_slot_double(1, g.1.wren_id as f64)
                            });
                            let _ = self.vm.call_handle(&static_update_function);
                        }
                    }
                }
            } 

            self.receive_audio(app, state);
            self.handle_timer(app, state);
            self.receive_state(app, state);
        }
    }
}

/// App wrapper
pub struct App {
    pub canvas: Canvas<Window>,
    pub font_context: Sdl2TtfContext,
    pub input: Input,
    pub time: Timer, 
    pub tex_creator : TextureCreator<WindowContext>,
    event_pump : EventPump,
    _audio_context : AudioSubsystem,
}

impl App {
    pub fn new(window_title : &str, window_size: Vec2) -> Self {
        let sdl_ctx: Sdl = sdl2::init().unwrap();
        let font_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG).unwrap();
        let audio_context = sdl_ctx.audio().unwrap();

        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
        let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3 | sdl2::mixer::InitFlag::FLAC | sdl2::mixer::InitFlag::OGG).unwrap();
        sdl2::mixer::allocate_channels(128);

        let video_subsystem = sdl_ctx.video().unwrap();
        let win: Window = video_subsystem.window(window_title, window_size.x as u32, window_size.y as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canv: Canvas<Window> = win.into_canvas().present_vsync().build().unwrap();
        
        let event_pump = sdl_ctx.event_pump().unwrap();
        let tc = canv.texture_creator();

        Self {
            canvas: canv, 
            event_pump, 
            input: Input::new(),
            tex_creator : tc,
            time: Timer::new(),
            font_context,
            _audio_context: audio_context,
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        match self.canvas.window().fullscreen_state() {
            FullscreenType::Off => {
                if let Err(e) = self.canvas.window_mut().set_fullscreen(FullscreenType::Desktop) {
                    LilahPanic!(App, e);
                }
            }
            FullscreenType::True => {
                if let Err(e) = self.canvas.window_mut().set_fullscreen(FullscreenType::Off) {
                    LilahPanic!(App, e);
                }
            }
            FullscreenType::Desktop => {
                if let Err(e) = self.canvas.window_mut().set_fullscreen(FullscreenType::Off) {
                    LilahPanic!(App, e);
                }
            }
        }
    }

    pub fn get_fullscreen(&self) -> bool {
        match self.canvas.window().fullscreen_state() {
            FullscreenType::Off => {
                false
            }
            FullscreenType::True => {
                true
            }
            FullscreenType::Desktop => {
                true
            }
        }
    }

    pub fn set_fullscreen(&mut self, set: bool) {
        match set {
            true => {
                if let Err(e) = self.canvas.window_mut().set_fullscreen(FullscreenType::Desktop) {
                    LilahPanic!(App, e);
                }
            }
            false => {
                if let Err(e) = self.canvas.window_mut().set_fullscreen(FullscreenType::Off) {
                    LilahPanic!(App, e);
                }
            }
        }
    }

    pub fn delta_time(&self) -> f32 {
        self.time.delta_time
    }

    pub fn handle_input(&mut self) -> bool {
        let mut end_app: bool = false;

        let mouse_pos = Vec2::new(self.event_pump.mouse_state().x() as f64, self.event_pump.mouse_state().y() as f64);
        self.input.update_mouse_pos(mouse_pos);

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                    end_app = true;
                },
                Event::MouseButtonDown { 
                    mouse_btn,
                    ..
                } => {
                    self.input.update_mouse((&mouse_btn, &InputInfo{pressed: true, pressed_down: true}));
                }
                Event::MouseButtonUp { 
                    mouse_btn,
                    ..
                } => {
                    self.input.update_mouse((&mouse_btn, &InputInfo{pressed: false, pressed_down: false}));
                }
                Event::KeyDown {
                keycode,
                repeat: false,
                ..
                } => {
                    self.input.update_mapping((&keycode.unwrap(), &InputInfo{pressed: true, pressed_down: true}));
                },
                Event::KeyUp {
                keycode,
                repeat: false,
                ..
                } => {
                    self.input.update_mapping((&keycode.unwrap(), &InputInfo{pressed: false, pressed_down: false}));
                },
                
                _ => {}
            }
        }
        return end_app
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    /// Performs time update, canvas clear, and handles input.
    pub fn pre_frame(&mut self) -> bool {
        self.time.update();
        self.canvas.clear();
        self.handle_input()
    }

    /// Draws canvas and sleeps until next frame
    pub fn present_frame(&mut self) {
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

impl Scripting {
    pub fn handle_input(&self, app : &mut App, _state : &mut WorldState) {
        self.vm.execute(|vm| {
            vm.ensure_slots(1);
            vm.get_variable("app", "Input", 0);
        });
        let class = self.vm.get_slot_handle(0);

        for entry in &app.input.mappings {
            self.vm.set_slot_handle(0, &class);
            let sm = self.vm.make_call_handle(FunctionSignature::new_function("update_mapping", 3));
            let a = entry.0.to_string();
            let b = entry.1.pressed;
            let c = entry.1.pressed_down;
            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
                vm.set_slot_bool(2, b);
                vm.set_slot_bool(3, c);
            });
            self.vm.set_slot_handle(0, &class);
            let _ = self.vm.call_handle(&sm);
        }

        for entry in &app.input.mouse_mapping {
            self.vm.set_slot_handle(0, &class);
            let sm = self.vm.make_call_handle(FunctionSignature::new_function("update_mouse_mapping", 3));
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
            let _ = self.vm.call_handle(&sm);
        }

        for entry in &app.input.bindings {
            self.vm.set_slot_handle(0, &class);
            let sb = self.vm.make_call_handle(FunctionSignature::new_function("update_binding", 3));
            let a = entry.0.to_string();
            let b = entry.1.negative.to_string();
            let c = entry.1.positive.to_string();

            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
                vm.set_slot_string(2, b);
                vm.set_slot_string(3, c);
            });
            let _ = self.vm.call_handle(&sb);
        }

        self.vm.set_slot_handle(0, &class);
        let set_mouse = self.vm.make_call_handle(FunctionSignature::new_function("set_mouse_pos", 1));
        self.vm.execute(|vm| {
            let _ = vm.set_slot_new_foreign("math", "Vec2", app.input.mouse_pos, 1);
        });
        self.vm.set_slot_handle(0, &class);
        let _ = self.vm.call_handle(&set_mouse);
    }

    pub fn handle_timer(&self, app : &mut App, _state : &mut WorldState) {
        self.vm.execute(|vm| {
            vm.ensure_slots(2);
            vm.get_variable("app", "State", 0); 
        });

        let st = self.vm.make_call_handle(FunctionSignature::new_setter("delta_time"));
        let val = app.time.delta_time as f64;

        self.vm.execute(|vm| {
            vm.set_slot_double(1, val);
        });
        let _ = self.vm.call_handle(&st);
    }

    pub fn send_state(&self, app : &mut App, state : &mut WorldState) {
        self.vm.execute(|vm| {
            vm.get_variable("app", "State", 0);
        });
        let class = self.vm.get_slot_handle(0);
      
        self.vm.set_slot_handle(0, &class);
        let set_gs = self.vm.make_call_handle(FunctionSignature::new_setter("gameobjects"));

        self.vm.execute(|vm| {
            vm.set_slot_new_list(1);
            for i in state.gameobjects.iter().enumerate() {
                i.1.1.clone().send_to_wren(2, vm);
                vm.insert_in_list(1, i.0 as i32, 2);
            }
        });
        
        self.vm.set_slot_handle(0, &class);
        let _ = self.vm.call_handle(&set_gs);

        self.vm.set_slot_handle(0, &class);
        let set_fullscreen = self.vm.make_call_handle(FunctionSignature::new_setter("fullscreen"));

        self.vm.execute(|vm| {
            vm.set_slot_bool(1, app.get_fullscreen());
        });
        self.vm.set_slot_handle(0, &class);
        let _ = self.vm.call_handle(&set_fullscreen);

        self.vm.execute(|vm| {
            vm.get_variable("app", "Input", 0);
        });
        let input_class = self.vm.get_slot_handle(0);

        for entry in &mut app.input.mappings {
            self.vm.set_slot_handle(0, &input_class);
            let sm = self.vm.make_call_handle(FunctionSignature::new_function("is_pressed", 1));
            let a = entry.0.to_string();
            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
            });
            self.vm.set_slot_handle(0, &input_class);
            let _ = self.vm.call_handle(&sm);

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

        for entry in &mut app.input.mouse_mapping {
            self.vm.set_slot_handle(0, &input_class);
            let sm = self.vm.make_call_handle(FunctionSignature::new_function("is_mouse_pressed", 1));
            let a = match entry.0 {
                MouseButton::Left => "Left",
                MouseButton::Middle => "Middle",
                MouseButton::Right => "Right",
                _ => "Unknown",
            };
            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
            });
            self.vm.set_slot_handle(0, &input_class);
            let _ = self.vm.call_handle(&sm);
            let mut b = entry.1.pressed_down;    
            self.vm.execute(|vm| {
                if let Some(pressed) = vm.get_slot_bool(0) {
                    b = pressed;
                }
            });
            entry.1.pressed_down = b;
        }
    }

    pub fn receive_audio(&self, _app : &mut App, state : &mut WorldState) {
        self.vm.execute(|vm| {
            vm.ensure_slots(1);
            vm.get_variable("app", "Audio", 0);
        });
        let audio_class = self.vm.get_slot_handle(0);

        //recieve audio
        self.vm.set_slot_handle(0, &audio_class);
        let _ = self.vm.call(FunctionSignature::new_getter("dirty"));
        let mut dirty = false;
        self.vm.execute(|vm| {                
            if let Some(d) = vm.get_slot_bool(0) {
                dirty = d;
            }
        });
        if dirty {
            self.vm.set_slot_handle(0, &audio_class);
            let _ = self.vm.call(FunctionSignature::new_getter("command"));

            let mut command = String::from("");
            self.vm.execute(|vm| {                
                if let Some(cmd) = vm.get_slot_string(0) {
                    command = cmd;
                }
            });

            self.vm.set_slot_handle(0, &audio_class);
            let _ = self.vm.call(FunctionSignature::new_getter("volume"));

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
                    self.vm.set_slot_handle(0, &audio_class);
                    let _ = self.vm.call(FunctionSignature::new_getter("fade"));
                    let mut fade = 0;
                    self.vm.execute(|vm| {                
                        if let Some(f) = vm.get_slot_double(0) {
                            fade = f as i32;
                        }
                    });
                    let _ = sdl2::mixer::Music::fade_out(fade); 
                }
                "start" => {
                    self.vm.set_slot_handle(0, &audio_class);
                    let _ = self.vm.call(FunctionSignature::new_getter("music"));
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
                    self.vm.set_slot_handle(0, &audio_class);
                    let _ = self.vm.call(FunctionSignature::new_getter("music"));
                    let mut song = String::from("");

                    self.vm.execute(|vm| {                
                        if let Some(file) = vm.get_slot_string(0) {
                            song = file;
                        }
                    });

                    self.vm.set_slot_handle(0, &audio_class);
                    let _ = self.vm.call(FunctionSignature::new_getter("fade"));
                    let mut fade = 0;
                    self.vm.execute(|vm| {                
                        if let Some(f) = vm.get_slot_double(0) {
                            fade = f as i32;
                        }
                    });

                    if let Some(music) = state.music.get(&song) {
                        sdl2::mixer::Music::halt();
                        let _ = music.fade_in(-1, fade);
                    }  
                }
                _ => {}
            }
        }
        else {
            self.vm.execute(|vm| {                
                vm.set_slot_double(1, sdl2::mixer::Music::get_volume() as f64);
            });

            self.vm.set_slot_handle(0, &audio_class);
            let _ = self.vm.call(FunctionSignature::new_setter("volume"));
        }
        //end
    }

    pub fn receive_state(&self, app : &mut App, state : &mut WorldState) {
        //get classes
        self.vm.execute(|vm| {
            vm.ensure_slots(1);
            vm.get_variable("app", "State", 0);
        });
        let state_class = self.vm.get_slot_handle(0);
        //end

        //recieve gameobjects
        self.vm.set_slot_handle(0, &state_class);
        let _ = self.vm.call(FunctionSignature::new_getter("gameobjects"));

        self.vm.execute(|vm| {
            if let Some(count) = vm.get_list_count(0) {
                for i in 0..count {
                    vm.get_list_element(0, i as i32, 1);
                    
                    let go = vm.get_slot_foreign::<GameObject>(1);
                    if let Some(g) = go {
                        let mut  g2 = g.clone();
                        g2.wren_id = i;
                        state.insert_wren(g2.clone());
                    }
                }
            }
        });

        self.vm.set_slot_handle(0, &state_class);
        let _ = self.vm.call(FunctionSignature::new_getter("destroy"));

        self.vm.execute(|vm| {
            if let Some(count) = vm.get_list_count(0) {
                for i in 0..count {
                    vm.get_list_element(0, i as i32, 1);
                    
                    let go = vm.get_slot_foreign::<GameObject>(1);
                    if let Some(g) = go {
                        state.gameobjects.remove(&g.id.uuid);
                    }
                }
            }
        });
        //end

        self.vm.set_slot_handle(0, &state_class);
        let get_fullscreen = self.vm.make_call_handle(FunctionSignature::new_getter("fullscreen"));
        let _ = self.vm.call_handle(&get_fullscreen);

        self.vm.execute(|vm| {
            if let Some(is_sfull) = vm.get_slot_bool(0) {
                if is_sfull != app.get_fullscreen() {
                    app.toggle_fullscreen();
                }
            }
        });

        //clear all
        self.vm.set_slot_handle(0, &state_class);
        let _ = self.vm.call(FunctionSignature::new_function("clear", 0));
        //end
    }
}