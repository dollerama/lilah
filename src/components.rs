use ruwren::{Class, VM, send_foreign, create_module, ModuleLibrary};
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::rwops::RWops;
use sdl2::{render::Texture, rect::Rect};
use uuid::Uuid;
use crate::{LilahTypeError, LilahNotFoundError, LilahError, LilahTypePanic, LilahPanic};
use crate::gameobject::GameObjectId;
use crate::world::StateUpdateContainer;
use crate::{application::App, gameobject::GameObject};
use crate::math::Vec2;
use std::{any::Any, collections::HashMap};

/// Tick/Update Component 
pub trait Tickable<T: Component> {
    ///Tick Component with delta time and Component
    /// # Example
    /// impliments tickable for Transform that depends on Rigidbody. This snippet will set the transform to the rigidbody position when ticked.
    /// ```rust, no_run
    /// impl Tickable<Rigidbody> for Transform {
    ///     fn tick(&mut self, _: f32, d: &Rigidbody) {
    ///         self.position = d.position;
    ///     }
    /// }
    /// ```
    fn tick(&mut self, dt: f32, d: &T);
}

pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn send_to_wren(&self, slot : usize, vm : &VM);
    fn clone_dyn(&self) -> Box<dyn Component>;
}

/// Transform Component for GameObjects
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f64,
}

/// Sfx Component for GameObjects
#[derive(Clone)]
pub struct Sfx {
    pub name: String,
    pub file: String,
    pub play_state: bool,
    pub volume: f64,
    pub channel: Option<sdl2::mixer::Channel>
}

/// Rigidbody Component for GameObjects
#[derive(PartialEq, Clone)]
pub struct Rigidbody {
    pub position: Vec2,
    /// Bounds of Collider
    pub bounds: Vec2,
    pub velocity: Vec2,
    /// GameObjectID of current collider 
    pub colliding : Option<GameObjectId>,
    /// If set to false colliding is still populated but the rigidbody will not correct its velocity when collisions are detected.
    pub solid : bool
}

/// Sprite Component for GameObjects
#[derive(Clone)]
pub struct Sprite {
    /// size of sprite sheet
    base_size: (u32, u32),
    /// Start position on sprite sheet
    index_cut: (i32, i32),
    /// size of sprite cell
    size: (u32, u32),
    /// Current position on sprite sheet
    index: (i32, i32),
    /// Texture file name
    pub texture_id: String
}

/// Animator Component for GameObjects
#[derive(PartialEq, Default, Clone)]
pub struct Animator {
    /// Name of State(String), sprite sheet index(i32, i32)
    states: HashMap<String, (i32, i32)>,
    current_state: String,
    pub current_frame: f64,
    pub speed: f64,
    playing: bool
}

/// Behaviour Component for GameObjects
#[derive(Clone)]
pub struct ComponentBehaviour {
    /// Name of wren class to link to behaviour
    component: String
}

/// Text Component for GameObjects
#[derive(Clone)]
pub struct Text {
    /// Name of wren class to link to behaviour
    text: String,
    font_size: u32,
    font: String,
    texture_id: String,
    changed: bool,
    size: Vec2
}

//component impls
impl Sfx {
    pub fn new(name: String, file: String) -> Self {
        Self {
            name,
            file,
            play_state: false,
            volume: 128.0,
            channel: None
        }
    }

    pub fn play(&mut self) {
        self.play_state = true;
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "engine", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_name_getter(&self, vm: &VM) {
        vm.set_slot_string(0, self.name.clone());
    }

    fn wren_name_setter(&mut self, vm: &VM) {
        match vm.get_slot_string(1) {
            Some(name) => self.name = name.clone(),
            None => { eprintln!("Sfx Error: Arg (1) must be of type String"); }
        }
    }

    fn wren_file_getter(&self, vm: &VM) {
        vm.set_slot_string(0, self.file.clone());
    }

    fn wren_volume_getter(&self, vm: &VM) {
        vm.set_slot_double(0, self.volume);
    }

    fn wren_volume_setter(&mut self, vm: &VM) {
        match vm.get_slot_double(1) {
            Some(volume) => self.volume = volume,
            None => { eprintln!("Sfx Error: Arg (1) must be of type Double"); }
        }
    }

    fn wren_play(&mut self, _vm: &VM) {
        self.play_state = true;
    }

    fn wren_set_volume_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                let name = vm.get_slot_string(2);
                let vol = vm.get_slot_double(3);
                if let (Some(n), Some(v)) = (name, vol) {
                    for i in comp.wrap_all_mut::<Sfx>() {
                        if i.name == n {
                            i.volume = v;
                            break;
                        }
                    }
                }
                else {
                    eprintln!("Sfx Error: Arg (2) must be of type String and Arg (3) must be of type Double");
                }
            }
            None => {
                eprintln!("Sfx Error: Arg (1) must be of type GameObject");
            }
        }
    }

    fn wren_play_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                let name = vm.get_slot_string(2);
                if let Some(n) = name {
                    for i in comp.wrap_all_mut::<Sfx>() {
                        if i.name == n {
                            i.play_state = true;
                            break;
                        }
                    }
                }
                else {
                    eprintln!("Sfx Error: Arg (2) must be of type String");
                }
            }
            None => {
                eprintln!("Sfx Error: Arg (1) must be of type GameObject");
            }
        }
    }

    fn wren_get_volume_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                let name = vm.get_slot_string(2);
                if let Some(n) = name {
                    for i in comp.wrap_all_mut::<Sfx>() {
                        if i.name == n {
                            vm.set_slot_double(0, i.volume);
                            break;
                        }
                    }
                }
                else {
                    eprintln!("Sfx Error: Arg (2) must be of type String");
                }
            }
            None => {
                eprintln!("Sfx Error: Arg (1) must be of type GameObject");
            }
        }
    }
}

//component impls
impl Transform {
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            rotation: 0.0,
            scale: Vec2::ONE
        }
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "engine", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_get_pos(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.position => 0);
    }

    fn wren_get_scale(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.scale => 0);
    }

    fn wren_get_rotation(&self, vm: &VM) {
        vm.set_slot_double(0, self.rotation);
    }

    fn wren_set_pos(&mut self, vm: &VM) {
        match vm.get_slot_foreign::<Vec2>(1) {
            Some(pos) => self.position = *pos,
            None => { LilahTypeError!(Transform, 1, Vec2); }
        }
    }

    fn wren_set_scale(&mut self, vm: &VM) {
        match vm.get_slot_foreign::<Vec2>(1) {
            Some(scale) => self.scale = *scale,
            None => { LilahTypeError!(Transform, 1, Vec2); }
        }
    }

    fn wren_set_rotation(&mut self, vm: &VM) {
        match vm.get_slot_double(1) {
            Some(rotation) => self.rotation = rotation,
            None => { LilahTypeError!(Transform, 1, f64); }
        }
    }

    fn wren_set_pos_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(pos) => comp.get_mut::<Transform>().position = *pos,
                    None => { LilahTypeError!(Transform, 2, Vec2); }
                }
            }
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(pos_x) => comp.get_mut::<Transform>().position.x = pos_x,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }

    fn wren_set_pos_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(pos_y) => comp.get_mut::<Transform>().position.y = pos_y,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }

    fn wren_update_pos_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(pos) => comp.get_mut::<Transform>().position += *pos,
                    None => { LilahTypeError!(Transform, 2, Vec2);  }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }

    fn wren_update_pos_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(pos_x) => comp.get_mut::<Transform>().position.x += pos_x,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }

    fn wren_update_pos_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(pos_y) => comp.get_mut::<Transform>().position.y += pos_y,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }

    fn wren_set_scale_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(scale) => comp.get_mut::<Transform>().scale = *scale,
                    None => { LilahTypeError!(Transform, 2, Vec2); }
                }
            }
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_scale_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(scale_x) => comp.get_mut::<Transform>().scale.x = scale_x,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }

    fn wren_set_scale_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(scale_y) => comp.get_mut::<Transform>().scale.y = scale_y,
                    None => { LilahTypeError!(Transform, 2, f64);  }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }

    fn wren_update_scale_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(scale) => comp.get_mut::<Transform>().scale += *scale,
                    None => { LilahTypeError!(Transform, 2, Vec2);  }
                }
            }
            None => {
                LilahTypeError!(Transform, 1, GameObject); 
            }
        }
    }

    fn wren_update_scale_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(scale_x) => comp.get_mut::<Transform>().scale.x += scale_x,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject);  }
        }
    }

    fn wren_update_scale_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(scale_y) => comp.get_mut::<Transform>().scale.y += scale_y,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject);  }
        }
    }

    fn wren_set_rot_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(rotation) => comp.get_mut::<Transform>().rotation = rotation,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject);  }
        }
    }

    fn wren_update_rot_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(rotation) => comp.get_mut::<Transform>().rotation += rotation,
                    None => { LilahTypeError!(Transform, 2, f64); }
                }
            }
            None => { LilahTypeError!(Transform, 1, GameObject); }
        }
    }
    //for wren
}

impl Rigidbody {
    pub fn new(pos: Vec2) -> Self {
        Self {
            bounds : Vec2::ONE,
            velocity : Vec2::ZERO,
            position : pos,
            colliding : None,
            solid : true
        }
    }

    pub fn new_without_pos() -> Self {
        Self {
            bounds : Vec2::ONE,
            velocity : Vec2::ZERO,
            position : Vec2::ZERO,
            colliding : None,
            solid : true
        }
    }

    pub fn update_vel_y(&mut self) {
        self.position.y += self.velocity.y; 
    }

    pub fn update_vel_x(&mut self) {
        self.position.x += self.velocity.x; 
    }

    pub fn update_correct_y(&mut self) {
        self.position.y -= self.velocity.y; 
    }

    pub fn update_correct_x(&mut self) {
        self.position.x -= self.velocity.x; 
    }

    /// Simple AABB collision
    pub fn check_collision(&self, other: &Rigidbody) -> bool {
        //The sides of the rectangles
        let left_a = self.position.x;
        let left_b = other.position.x;
        let right_a = self.position.x+self.bounds.x;
        let right_b = other.position.x+other.bounds.x;
        let top_a = self.position.y;
        let top_b = other.position.y;
        let bottom_a = self.position.y+self.bounds.y;
        let bottom_b = other.position.y+other.bounds.y;

        //If any of the sides from A are outside of B
        if  bottom_a >= top_b &&
            top_a <= bottom_b &&
            right_a >= left_b &&
            left_a <= right_b {
            return true;
        }

        false
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "engine", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_vel_getter(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.velocity => 0);
    }

    fn wren_vel_setter(&mut self, vm: &VM) {
        match vm.get_slot_foreign::<Vec2>(1) {
            Some(vel) => self.velocity = *vel,
            None => { LilahTypeError!(Rigidbody, 1, Vec2); }
        }
    }

    fn wren_solid_getter(&self, vm: &VM) {
        vm.set_slot_bool(0, self.solid);
    }

    fn wren_solid_setter(&mut self, vm: &VM) {
        match vm.get_slot_bool(1) {
            Some(solid) => self.solid = solid,
            None => { LilahTypeError!(Rigidbody, 1, bool); }
        }
    }

    fn wren_colliding_getter(&mut self, vm: &VM) {
        if let Some(coll) = self.colliding.clone() {
            vm.set_slot_new_map(0);
            vm.set_slot_string(1, "id");
            vm.set_slot_string(2, coll.name.clone());
            vm.set_map_value(0, 1, 2);
            vm.set_slot_string(1, "uuid");
            vm.set_slot_string(2, coll.uuid.clone());
            vm.set_map_value(0, 1, 2);
        }
        else {
            vm.set_slot_null(0)
        }
    }

    fn wren_colliding_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                if let Some(coll) = comp.get::<Rigidbody>().colliding.clone() {
                    vm.set_slot_new_map(0);
                    vm.set_slot_string(2, "id");
                    vm.set_slot_string(3, coll.name.clone());
                    vm.set_map_value(0, 2, 3);
                    vm.set_slot_string(2, "uuid");
                    vm.set_slot_string(3, coll.uuid.clone());
                    vm.set_map_value(0, 2, 3);
                }
                else {
                    vm.set_slot_null(0)
                }
            }
            None => { LilahTypeError!(Rigidbody, 1, GameObject); }
        }
    }

    fn wren_set_vel_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(vel) => comp.get_mut::<Rigidbody>().velocity = *vel,
                    None => { LilahTypeError!(Rigidbody, 2, Vec2); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(pos) => comp.get_mut::<Rigidbody>().position = *pos,
                    None => { LilahTypeError!(Rigidbody, 2, Vec2); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(pos_x) => comp.get_mut::<Rigidbody>().position.x = pos_x,
                    None => { LilahTypeError!(Rigidbody, 2, f64); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(pos_y) => comp.get_mut::<Rigidbody>().position.y = pos_y,
                    None => { LilahTypeError!(Rigidbody, 2, f64); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_vel_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(vel_x) => comp.get_mut::<Rigidbody>().velocity.x = vel_x,
                    None => { LilahTypeError!(Rigidbody, 2, f64); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_vel_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(vel_y) => comp.get_mut::<Rigidbody>().velocity.y = vel_y,
                    None => { LilahTypeError!(Rigidbody, 2, f64); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_solid_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_bool(2) {
                    Some(solid) => comp.get_mut::<Rigidbody>().solid = solid,
                    None => { LilahTypeError!(Rigidbody, 2, bool); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_update_vel_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(vel) => comp.get_mut::<Rigidbody>().velocity += *vel,
                    None => { LilahTypeError!(Rigidbody, 2, Vec2); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_update_vel_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(vel_x) => comp.get_mut::<Rigidbody>().velocity.x += vel_x,
                    None => { LilahTypeError!(Rigidbody, 2, f64); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_update_vel_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(vel_y) => comp.get_mut::<Rigidbody>().velocity.y += vel_y,
                    None => { LilahTypeError!(Rigidbody, 2, f64); }
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }
}

impl Animator {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            current_state: String::from("None"),
            current_frame: 0.0,
            speed: 10.0,
            playing: false
        }
    }

    pub fn with(mut self, key: &str, i: i32, j: i32) -> Animator {
        self.states.insert(key.to_string(), (i, j));
        self
    }

    pub fn at_speed(mut self, s: f64) -> Animator {
        self.speed = s;
        self
    }

    pub fn at_frame(mut self, s: f64) -> Animator {
        self.current_frame = s;
        self
    }

    pub fn start_with(mut self, key: &str) -> Animator {
        self.set_state(key);
        self
    }

    pub fn build_playing(mut self) -> Animator {
        self.playing = true;
        self
    }
    
    pub fn build(self) -> Animator {
        self
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }
    
    pub fn set_state(&mut self, st: &str) {    
        match self.states.get(&st.to_string()) {
            Some(_) => self.current_state = st.to_string(),
            None => {} 
        }
    }

    pub fn get_state(&self) -> &String {
        &self.current_state
    }

    pub fn insert_state(&mut self, key: &str, i: i32, j: i32) {
        self.states.insert(key.to_string(), (i, j));
    }

    pub fn update(&mut self, dt: f32) {
        if self.playing && self.current_state != String::from("None") {
            if self.current_frame > self.states.get(&self.current_state).unwrap().0 as f64 {
                self.current_frame = 0.0;
            }

            self.current_frame += dt as f64*self.speed;

            if self.current_frame > self.states.get(&self.current_state).unwrap().0 as f64 {
                self.current_frame = 0.0;
            }
        }
    }

    pub fn update_sprite(&self, sprite: &mut Sprite) {
        if self.current_state != String::from("None") {
            sprite.anim_sprite_sheet(
                self.current_frame as i32, 
                self.states.get(&self.current_state).unwrap().1
            );
        }
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "engine", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_playing_getter(&self, vm: &VM) {
        vm.set_slot_bool(0, self.playing);   
    }

    fn wren_frame_getter(&self, vm: &VM) {
        vm.set_slot_double(0, self.current_frame as f64);   
    }

    fn wren_speed_getter(&self, vm: &VM) {
        vm.set_slot_double(0, self.speed);   
    }

    fn wren_play(&mut self, _vm: &VM) {
        self.play();  
    }

    fn wren_stop(&mut self, _vm: &VM) {
        self.stop();  
    }

    fn wren_get_state(&self, vm: &VM) {
        match vm.get_slot_string(1) {
            Some(state) => {
                match self.states.get(&state) {
                    Some(s) => {
                        vm.set_slot_new_map(0);
                        vm.set_slot_string(1, state);
                        send_foreign!(vm, "math", "Vec2", Vec2::new(s.0 as f64, s.1 as f64) => 2);
                        vm.set_map_value(0, 1, 2);
                    }
                    None => {
                        LilahNotFoundError!(Animator, String, state);
                    }
                }
            }
            None => {
                LilahTypeError!(Animator, 1, String);
            }
        }
    }

    fn wren_set_state(&mut self, vm: &VM) {
        match vm.get_slot_string(1) {
            Some(state) => {
                self.set_state(&state);
            }
            None => {
                LilahTypeError!(Animator, 1, String);
            }
        }
    }

    fn wren_insert_state(&mut self, vm: &VM) {
        match vm.get_slot_string(1) {
            Some(state) => {
                match vm.get_slot_foreign::<Vec2>(2) {
                    Some(loc) => {
                        self.states.insert(state, (loc.x as i32, loc.y as i32));
                    }
                    None => {
                        LilahTypeError!(Animator, 2, Vec2);
                    }
                }
            }
            None => {
                LilahTypeError!(Animator, 1, String);
            }
        }
    }

    fn wren_set_speed(&mut self, vm: &VM) {
        match vm.get_slot_double(1) {
            Some(speed) => {
                self.speed = speed;
            }
            None => {
                LilahTypeError!(Animator, 1, f64);
            }
        }
    }

    fn wren_set_frame(&mut self, vm: &VM) {
        match vm.get_slot_double(1) {
            Some(frame) => {
                self.current_frame = frame;
            }
            None => {
                LilahTypeError!(Animator, 1, f64);
            }
        }
    }

    fn wren_play_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                comp.get_mut::<Animator>().play();
            }
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_stop_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                comp.get_mut::<Animator>().stop();
            }
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_set_state_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_string(2) {
                    Some(state) => {
                        comp.get_mut::<Animator>().set_state(&state);
                    }
                    None => {
                        LilahTypeError!(Animator, 2, String);
                    }
                }
            }
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_get_state_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_string(2) {
                    Some(state) => {
                        match comp.get::<Animator>().states.get(&state) {
                            Some(s) => {
                                vm.set_slot_new_map(0);
                                vm.set_slot_string(1, state);
                                send_foreign!(vm, "math", "Vec2", Vec2::new(s.0 as f64, s.1 as f64) => 2);
                                vm.set_map_value(0, 1, 2);
                            }
                            None => {
                                LilahNotFoundError!(Animator, String, state);
                            }
                        }
                    }
                    None => {
                        LilahTypeError!(Animator, 2, String);
                    }
                }
            }
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_insert_state_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_string(2) {
                    Some(state) => {
                        match vm.get_slot_foreign::<Vec2>(3) {
                            Some(loc) => {
                                comp.get_mut::<Animator>().states.insert(state, (loc.x as i32, loc.y as i32));
                            }
                            None => {
                                LilahTypeError!(Animator, 3, Vec2);
                            }
                        }
                    }
                    None => {
                        LilahTypeError!(Animator, 2, String);
                    }
                }
            }
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_set_speed_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(speed) => {
                        comp.get_mut::<Animator>().speed = speed;
                    }
                    None => {
                        LilahTypeError!(Animator, 2, f64);
                    }
                }
            }
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_set_frame_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                match vm.get_slot_double(2) {
                    Some(frame) => {
                        comp.get_mut::<Animator>().current_frame = frame;
                    }
                    None => {
                        LilahTypeError!(Animator, 2, f64);
                    }
                }
            }
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }
}

impl Text {
    pub fn new(t: &str, font: &str) -> Self {
        Self {
            text: t.to_string(),
            font_size: 24,
            font: font.to_string(),
            texture_id: Uuid::new_v4().to_string(),
            changed: true,
            size: Vec2::ZERO
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, t: &str) {
        self.text = t.to_string();
        self.changed = true;
    }

    pub fn get_font(&self) -> &String {
        &self.font
    }

    pub fn set_font(&mut self, t: &str) {
        self.font = t.to_string();
        self.changed = true;
    }

    pub fn get_font_size(&self) -> u32 {
        self.font_size
    }

    pub fn set_font_size(&mut self, s: u32) {
        self.font_size = s;
        self.changed = true;
    }

    pub fn load(&mut self, app: &mut App, fonts: &HashMap<String, Vec<u8>>) -> StateUpdateContainer {
        if self.changed {
            self.changed = false;

            if let Some(font_bytes) = fonts.get(&self.font) {
                let font = 
                app.font_context.load_font_from_rwops(
                    RWops::from_bytes(&font_bytes).unwrap(), 
                    self.font_size.try_into().unwrap()
                ).unwrap();

                let surface = font
                    .render(&self.text)
                    .blended(Color::RGBA(255, 255, 255, 255))
                    .map_err(|e| e.to_string()).unwrap();
                let texture = app.tex_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string()).unwrap();

                let TextureQuery { width, height, .. } = texture.query();
                self.size = Vec2::new(width as f64, height as f64);

                StateUpdateContainer { textures: Some((self.texture_id.clone(), texture)), sfx: None }
            }
            else {
                let f = self.font.clone();
                LilahNotFoundError!(Text, String, f);
                StateUpdateContainer { textures: None, sfx: None  }
            }
        }
        else {
            StateUpdateContainer { textures: None, sfx: None }
        }
    }

    pub fn draw(&self, app: &mut App, textures: &HashMap<String, Texture>, t: &Transform, camera: &Option<Vec2>) {
        let mut c_x = 0;
        let mut c_y = 0;
        if let Some(cam_pos) = camera {
            c_x = cam_pos.x as i32;
            c_y = cam_pos.y as i32;
        }

        if let Err(e) = app.canvas.copy(
        &textures[&self.texture_id], 
        None, 
        Some(Rect::new(t.position.x as i32-c_x, t.position.y as i32-c_y, self.size.x as u32, self.size.y as u32))
        ) {
            LilahError!(Text, e);
        }
    }

    //wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "engine", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_get_text(&self, vm: &VM) {
        vm.set_slot_string(0, self.text.clone());
    }

    fn wren_get_font(&self, vm: &VM) {
        vm.set_slot_string(0, self.font.clone());
    }

    fn wren_get_font_size(&self, vm: &VM) {
        vm.set_slot_double(0, self.font_size as f64);
    }

    fn wren_set_text(&mut self, vm: &VM) {
        let a = vm.get_slot_string(1);
        if let Some(a) = a {
            self.set_text(&a);
        }
        else {
            LilahTypeError!(Text, 1, String);
        }
    }

    fn wren_set_font(&mut self, vm: &VM) {
        let a = vm.get_slot_string(1);
        if let Some(a) = a {
            self.set_font(&a);
        }
        else {
            LilahTypeError!(Text, 1, String);
        }
    }

    fn wren_set_font_size(&mut self, vm: &VM) {
        let a = vm.get_slot_double(1);
        if let Some(a) = a {
            self.set_font_size(a as u32);
        }
        else {
            LilahTypeError!(Text, 1, f64);
        }
    }

    fn wren_get_text_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            vm.set_slot_string(0, comp.get_mut::<Text>().get_text());
        }
        else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_get_font_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            vm.set_slot_string(0, comp.get_mut::<Text>().get_font());
        }
        else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_get_font_size_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            vm.set_slot_double(0, comp.get_mut::<Text>().get_font_size() as f64);
        }
        else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_set_text_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            let a = vm.get_slot_string(2);
            if let Some(a) = a {
                comp.get_mut::<Text>().set_text(&a);
            }
            else {
                LilahTypeError!(Text, 2, String);
            }
        }
        else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_set_font_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            let a = vm.get_slot_string(2);
            if let Some(a) = a {
                comp.get_mut::<Text>().set_font(&a);
            }
            else {
                LilahTypeError!(Text, 2, String);
            }
        }
        else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_set_font_size_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            let a = vm.get_slot_double(2);
            if let Some(a) = a {
                comp.get_mut::<Text>().set_font_size(a as u32);
            }
            else {
                LilahTypeError!(Text, 2, f64);
            }
        }
        else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }
}

impl Sprite {
    pub fn new(t_id: &str) -> Self {
        Self {
            size: (1, 1),
            base_size: (1, 1),
            index_cut: (0, 0),
            index: (0,0),
            texture_id: t_id.to_string()
        }
    }

    pub fn load(&mut self, textures: &HashMap<String, Texture>) {
        if let Some(t) = textures.get(&self.texture_id) {
            self.base_size = (
                t.query().width,
                t.query().height,
            );
        }
        else {
            let id = self.texture_id.clone();
            LilahNotFoundError!(Sprite, Texture, id);
        }

        self.anim_sprite_sheet(self.index_cut.0, self.index_cut.1);
    }

    pub fn get_size(&self) -> (u32, u32) {
        (
            self.base_size.0/self.size.0,
            self.base_size.1/self.size.1,
        )
    }

    pub fn cut_sprite_sheet(&mut self, ind: i32, ind2: i32, col: u32, row: u32) {
        self.size = (col,row);
        self.index_cut = (ind, ind2);
        self.index = (0, 0);
    }

    pub fn anim_sprite_sheet(&mut self, ind: i32, ind2: i32) {
        self.index = (ind*self.get_size().0 as i32, ind2*self.get_size().1 as i32);
    }

    pub fn draw(&self, app: &mut App, textures: &HashMap<String, Texture>, t: &Transform, camera: &Option<Vec2>) {
        let mut c_x = 0;
        let mut c_y = 0;
        if let Some(cam_pos) = camera {
            c_x = cam_pos.x as i32;
            c_y = cam_pos.y as i32;
        }

        if let Err(e) = app.canvas.copy_ex(
            &textures[&self.texture_id], 
            Rect::new(
                self.index.0,
                self.index.1,
                self.get_size().0,
                self.get_size().1 ,
            ), 
            Rect::new(
                t.position.x as i32 - c_x, t.position.y as i32 - c_y, 
                self.get_size().0*t.scale.x.abs() as u32, 
                self.get_size().1*t.scale.y.abs() as u32),
            t.rotation,
            None,
            t.scale.x < 0.0,
            t.scale.y < 0.0
        ) {
            LilahError!(Sprite, e);
        }
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "engine", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_get_size(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", Vec2::new(self.get_size().0 as f64, self.get_size().1 as f64) => 0);
    }

    fn wren_get_index(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", Vec2::new(self.index.0 as f64, self.index.1 as f64) => 0);
    }

    fn wren_cut_sprite_sheet(&mut self, vm: &VM) {
        if let (Some(xy), Some(colrow)) = (vm.get_slot_foreign::<Vec2>(1), vm.get_slot_foreign::<Vec2>(1)) {
            self.cut_sprite_sheet(xy.x as i32, xy.y as i32, colrow.x as u32, colrow.y as u32);
        }
        else {
            LilahTypeError!(Sprite, 2, Vec2);
            LilahTypeError!(Sprite, 3, Vec2);
        }
    }


    fn wren_get_texture_id(&self, vm: &VM) {
        vm.set_slot_string(0, self.texture_id.clone());
    }

    fn _wren_set_size_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            if let Some(pos) = vm.get_slot_foreign::<Vec2>(2) {
                comp.get_mut::<Sprite>().base_size = (pos.x as u32, pos.y as u32);
            }
            else {
                LilahTypeError!(Sprite, 2, Vec2);
            }
        }
        else {
            LilahTypeError!(Sprite, 1, GameObject);
        }
    }

    fn wren_cut_sprite_sheet_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            if let (Some(xy), Some(colrow)) = (vm.get_slot_foreign::<Vec2>(2), vm.get_slot_foreign::<Vec2>(3)) {
                comp.get_mut::<Sprite>()
                .cut_sprite_sheet(xy.x as i32, xy.y as i32, colrow.x as u32, colrow.y as u32);
            }
            else {
                LilahTypeError!(Sprite, 2, Vec2);
                LilahTypeError!(Sprite, 3, Vec2);
            }
        }
        else {
            LilahTypeError!(Sprite, 1, GameObject);
        }
    }
}

impl ComponentBehaviour {
    pub fn new(s : String) -> Self {
        Self {
            component: s.clone()
        }
    }

    pub fn get_component(&self) -> &String {
        &self.component
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "engine", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        self.get_size() == other.get_size() && self.index == other.index
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            size: (u32::default(), u32::default()),
            index: (i32::default(), i32::default()),
            texture_id: "".to_string(),
            base_size: (u32::default(), u32::default()),
            index_cut: (i32::default(), i32::default())
        }
    }
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            bounds : Vec2::ONE,
            velocity : Vec2::ZERO,
            position : Vec2::ONE,
            colliding : None,
            solid : true    
        }
    }
}

//trait impl
impl Component for Transform {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "Transform", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for Sprite {
    fn as_any(& self) -> & dyn Any {
        self
    }
    fn as_any_mut(& mut self) -> & mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "Sprite", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for Rigidbody {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "Rigidbody", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for Animator {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "Animator", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for ComponentBehaviour {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "ComponentBehaviour", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for Text {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "Text", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for Sfx {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "Sfx", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Tickable<Sprite> for Rigidbody {
    fn tick(&mut self, _: f32, d: &Sprite) {
        let sprite_size = d.get_size();
        self.bounds = Vec2::new(sprite_size.0 as f64, sprite_size.1 as f64);
    }
}

impl Tickable<Rigidbody> for Transform {
    fn tick(&mut self, _: f32, d: &Rigidbody) {
        self.position = d.position;
    }
}

impl Tickable<Sprite> for Animator {
    fn tick(&mut self, dt: f32, _: &Sprite) {
        self.update(dt);
    }
}

impl Tickable<Animator> for Sprite {
    fn tick(&mut self, _: f32, d: &Animator) {
        d.update_sprite(self);
    }
}

//Class impls
impl Class for Box<dyn Component> {
    fn initialize(_: &VM) -> Self {
        LilahPanic!(Component, "Cannot instantiate static class");
    }
}

impl Class for Transform {
    fn initialize(vm: &VM) -> Transform {
        if let Some(pos) = vm.get_slot_foreign::<Vec2>(1) {
            Transform::new(*pos)
        }
        else {
            LilahTypePanic!(ComponentBehaviour, 1, Vec2);
        }
    }
}

impl Class for Sprite {
    fn initialize(vm: &VM) -> Sprite {
        if let Some(t_id) = vm.get_slot_string(1) {
            Sprite::new(t_id.as_str())
        }
        else {
            LilahTypePanic!(Sprite, 1, String);
        }
    }
}

impl Class for Rigidbody {
    fn initialize(_vm: &VM) -> Rigidbody {
        Rigidbody::new_without_pos()
    }
}

impl Class for ComponentBehaviour {
    fn initialize(vm: &VM) -> ComponentBehaviour {
        if let Some(c) = vm.get_slot_string(1) {
            ComponentBehaviour::new(c)
        }
        else {
            LilahTypePanic!(ComponentBehaviour, 1, String);
        }
    }
}

impl Class for Animator {
    fn initialize(_: &VM) -> Animator {
        Animator::new()
    }
}

impl Class for Sfx {
    fn initialize(vm: &VM) -> Sfx {
        if let (Some(b), Some(c)) = (vm.get_slot_string(1), vm.get_slot_string(2)) {
            Sfx::new(b, c)
        }
        else {
            LilahTypePanic!(Sfx, 1, String);
        }
    }
}

impl Class for Text {
    fn initialize(vm: &VM) -> Text {
        if let (Some(b), Some(c)) = (vm.get_slot_string(1), vm.get_slot_string(2)) {
            Text::new(b.as_str(), c.as_str())
        }
        else {
            LilahTypePanic!(Text, 1, String);
        }
    }
}

create_module! (
    class("Transform") crate::components::Transform => transform {
        instance(getter "as_component") wren_as_component,
        instance(getter "position") wren_get_pos,
        instance(getter "scale") wren_get_scale,
        instance(getter "rotation") wren_get_rotation,
        instance(setter "position") wren_set_pos,
        instance(setter "scale") wren_set_scale,
        instance(setter "rotation") wren_set_rotation,

        static(fn "set_position", 2) wren_set_pos_from_gameobject,
        static(fn "set_position_x", 2) wren_set_pos_x_from_gameobject,
        static(fn "set_position_y", 2) wren_set_pos_y_from_gameobject,
        static(fn "update_position", 2) wren_update_pos_from_gameobject,
        static(fn "update_position_x", 2) wren_update_pos_x_from_gameobject,
        static(fn "update_position_y", 2) wren_update_pos_y_from_gameobject,

        static(fn "set_scale", 2) wren_set_scale_from_gameobject,
        static(fn "set_scale_x", 2) wren_set_scale_x_from_gameobject,
        static(fn "set_scale_y", 2) wren_set_scale_y_from_gameobject,
        static(fn "update_scale", 2) wren_update_scale_from_gameobject,
        static(fn "update_scale_x", 2) wren_update_scale_x_from_gameobject,
        static(fn "update_scale_y", 2) wren_update_scale_y_from_gameobject,

        static(fn "set_rotation", 2) wren_set_rot_from_gameobject,
        static(fn "update_rotation", 2) wren_update_rot_from_gameobject
    }

    class("Sprite") crate::components::Sprite => sprite {
        instance(getter "as_component") wren_as_component,
        instance(getter "size") wren_get_size,
        instance(getter "texture_id") wren_get_texture_id,
        instance(getter "current_index") wren_get_index,
        instance(fn "cut_sprite_sheet", 2) wren_cut_sprite_sheet,
        static(fn "cut_sprite_sheet", 3) wren_cut_sprite_sheet_from_gameobject
    }

    class("Component") Box<dyn crate::components::Component> => component {
    }

    class("GameObject") crate::gameobject::GameObject => go {
        instance(fn "get_component", 1) wren_get_component,
        instance(fn "add_component", 1) wren_add_component,
        instance(fn "set_component", 2) wren_set_component,
        instance(getter "id") wren_getter_id,
        instance(getter "name") wren_getter_name,
        instance(setter "name") wren_setter_name,
        instance(getter "uuid") wren_getter_uuid
    }

    class("Rigidbody") crate::components::Rigidbody => rigidbody {
        instance(getter "as_component") wren_as_component,
        instance(getter "velocity") wren_vel_getter,
        instance(setter "velocity") wren_vel_setter,
        instance(getter "solid") wren_solid_getter,
        instance(setter "solid") wren_solid_setter,
        instance(getter "colliding") wren_colliding_getter,
        
        static(fn "colliding", 1) wren_colliding_from_gameobject,
        static(fn "set_velocity", 2) wren_set_vel_from_gameobject,
        static(fn "set_velocity_x", 2) wren_set_vel_x_from_gameobject,
        static(fn "set_velocity_y", 2) wren_set_vel_y_from_gameobject,
        static(fn "set_position", 2) wren_set_pos_from_gameobject,
        static(fn "set_position_x", 2) wren_set_pos_x_from_gameobject,
        static(fn "set_position_y", 2) wren_set_pos_y_from_gameobject,
        static(fn "update_velocity", 2) wren_update_vel_from_gameobject,
        static(fn "update_velocity_x", 2) wren_update_vel_x_from_gameobject,
        static(fn "update_velocity_y", 2) wren_update_vel_y_from_gameobject,
        static(fn "set_solid", 2) wren_set_solid_from_gameobject
    }

    class("Animator") crate::components::Animator => animator {
        instance(getter "as_component") wren_as_component,
        instance(getter "playing") wren_playing_getter,
        instance(getter "frame") wren_frame_getter,
        instance(getter "speed") wren_speed_getter,
        instance(setter "speed") wren_set_speed,
        instance(setter "frame") wren_set_frame,
        instance(fn "get_state", 1) wren_get_state,
        instance(fn "set_state", 1) wren_set_state,
        instance(fn "play", 0) wren_play,
        instance(fn "stop", 0) wren_stop,
        instance(fn "insert_state", 2) wren_insert_state,
        static(fn "play", 1) wren_play_from_gameobject,
        static(fn "stop", 1) wren_stop_from_gameobject,
        static(fn "set_state", 2) wren_set_state_from_gameobject,
        static(fn "get_state", 2) wren_get_state_from_gameobject,
        static(fn "insert_state", 3) wren_insert_state_from_gameobject,
        static(fn "set_speed", 2) wren_set_speed_from_gameobject,
        static(fn "set_frame", 2) wren_set_frame_from_gameobject
    }

    class("ComponentBehaviour") crate::components::ComponentBehaviour => component_behaviour {
        instance(getter "as_component") wren_as_component
    }

    class("Text") crate::components::Text => text {
        instance(getter "as_component") wren_as_component,
        instance(getter "text") wren_get_text,
        instance(getter "font") wren_get_font,
        instance(getter "font_size") wren_get_font_size,
        instance(setter "text") wren_set_text,
        instance(setter "font") wren_set_font,
        instance(setter "font_size") wren_set_font_size,
        static(fn "get_text", 1) wren_get_text_from_gameobject,
        static(fn "get_font", 1) wren_get_font_from_gameobject,
        static(fn "get_font_size", 1) wren_get_font_size_from_gameobject,
        static(fn "set_text", 2) wren_set_text_from_gameobject,
        static(fn "set_font", 2) wren_set_font_from_gameobject,
        static(fn "set_font_size", 2) wren_set_font_size_from_gameobject
    }

    class("Sfx") crate::components::Sfx => sfx {
        instance(getter "as_component") wren_as_component,
        instance(getter "name") wren_name_getter,
        instance(setter "name") wren_name_setter,
        instance(getter "volume") wren_volume_getter,
        instance(setter "volume") wren_volume_setter,
        instance(getter "file") wren_file_getter,
        instance(fn "play", 0) wren_play,
        static(fn "get_volume", 2) wren_get_volume_from_gameobject,
        static(fn "set_volume", 3) wren_set_volume_from_gameobject,
        static(fn "play", 2) wren_play_from_gameobject
    }

    module => engine
);

pub fn publish_modules(lib : &mut ModuleLibrary) {
    engine::publish_module(lib);
}