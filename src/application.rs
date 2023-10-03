extern crate sdl2;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;

use ruwren::{ModuleLibrary, VMConfig, VMWrapper, BasicFileLoader, ModuleScriptLoader, FunctionSignature};
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::{Sdl, EventPump};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use crate::components::ComponentBehaviour;
use crate::gameobject::GameObject;
use crate::input::{Input, InputInfo};
use crate::math::Vec2;
use crate::time::Timer;
use crate::world::WorldState;

#[macro_export]
macro_rules! load_script {
    ($path: expr, $scripting:ident) => {
        let module_str = $path.to_string();
        let module_name_full = module_str.split("/").collect::<Vec<&str>>();
        let mut module_name = module_name_full[module_name_full.len()-1];
        let module_name_final = module_name.split(".").collect::<Vec<&str>>();
        $scripting.load_script(module_name_final[0], include_str!($path));
    };
}
pub use load_script;

/// Scripting wrapper
pub struct Scripting {
    /// wren VM
    pub vm : VMWrapper,
    /// loaded modules. loaded as Module Name(String), Source(String).
    pub modules : HashMap<String, String>
}

impl Scripting {
    pub fn new() -> Self {
        let mut modules = HashMap::new();

        let binding = std::env::current_exe().unwrap();
        let current = Path::new(&binding).parent();
        let mut loader = BasicFileLoader::new().base_dir("");

        let mut lib = ModuleLibrary::new();

        crate::math::publish_modules(&mut lib);
        crate::components::publish_modules(&mut lib);

        let vm = VMConfig::new().enable_relative_import(true).library(&lib).script_loader(loader).build();

        vm.interpret("math", include_str!("scripts/math.wren")).unwrap();
        vm.interpret("app", include_str!("scripts/app.wren")).unwrap();
        vm.interpret("engine", include_str!("scripts/engine.wren")).unwrap();

        Self {
            vm : vm,
            modules : modules
        }
    }

    pub fn load_script(&mut self, module: &str, source: &str) {
        let mut src = source.to_string();
        let mod_name = module.to_string();
        src = format!("{}\nvar {} = {}.new()", src, mod_name.to_lowercase(), mod_name);
        self.modules.insert(mod_name.clone(), src.clone());
        self.vm.interpret(format!("{}Main", mod_name), src).unwrap();
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

            self.receive_state(app, state);
            self.handle_timer(app, state);
            self.handle_input(app, state);
        }
    }
}

/// App wrapper
pub struct App {
    pub canvas: Canvas<Window>,
    pub input: Input,
    pub time: Timer, 
    pub tex_creator : TextureCreator<WindowContext>,
    event_pump : EventPump,
}

impl App {
    pub fn new(window_title : &str) -> Self {
        let sdl_ctx: Sdl = sdl2::init().unwrap();
        //let font_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let video_subsystem = sdl_ctx.video().unwrap();
        let win: Window = video_subsystem.window(window_title, 800, 600)
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
        }
    }

    pub fn delta_time(&self) -> f32 {
        self.time.delta_time
    }

    pub fn handle_input(&mut self) -> bool {
        let mut end_app: bool = false;

        for i in &mut self.input.mappings {
            i.1.changed = false;
        }
        for i in &mut self.input.mouse_mapping {
            i.1.changed = false;
        }

        let mouse_pos = Vec2::new(self.event_pump.mouse_state().x() as f64, self.event_pump.mouse_state().y() as f64);
        self.input.update_mouse_pos(mouse_pos);

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    end_app = true;
                },
                Event::MouseButtonDown { 
                    mouse_btn,
                    ..
                } => {
                    self.input.update_mouse((&mouse_btn, &InputInfo{pressed: true, pressed_down: true, changed: true}));
                }
                Event::MouseButtonUp { 
                    mouse_btn,
                    ..
                } => {
                    self.input.update_mouse((&mouse_btn, &InputInfo{pressed: false, pressed_down: false, changed: true}));
                }
                Event::KeyDown {
                keycode,
                ..
                } => {
                    self.input.update_mapping((&keycode.unwrap(), &InputInfo{pressed: true, pressed_down: true, changed: true}));
                },
                Event::KeyUp {
                keycode,
                ..
                } => {
                    self.input.update_mapping((&keycode.unwrap(), &InputInfo{pressed: false, pressed_down: false, changed: true}));
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
    pub fn handle_input(&self, app : &mut App, state : &mut WorldState) {
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

        self.vm.set_slot_handle(0, &class);
        let sm = self.vm.make_call_handle(FunctionSignature::new_function("get_mapping", 1));

        for entry in &mut app.input.mappings {
            self.vm.set_slot_handle(0, &class);
            
            let a = entry.0.to_string();
            let mut input = entry.1.pressed_down;

            self.vm.execute(|vm| {
                vm.set_slot_string(1, a);
            });
            let _ = self.vm.call_handle(&sm);

            self.vm.execute(|vm| {
                vm.set_slot_string(1, "pressed_down");
                vm.get_map_value(0, 1, 2); 
                 
                if let Some(b) = vm.get_slot_bool(2) {
                    input = b;
                }            
            });
            entry.1.pressed_down = input;
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
        let a = self.vm.call_handle(&set_mouse);

        match a {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }
    }

    pub fn handle_timer(&self, app : &mut App, state : &mut WorldState) {
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
    }

    pub fn receive_state(&self, app : &mut App, state : &mut WorldState) {
        self.vm.execute(|vm| {
            vm.ensure_slots(1);
            vm.get_variable("app", "State", 0);
        });

        self.vm.set_slot_handle(0, &class);
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

        self.vm.set_slot_handle(0, &class);
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

        self.vm.set_slot_handle(0, &class);
        let _ = self.vm.call(FunctionSignature::new_function("load", 0));
    }
}