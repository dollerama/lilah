use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hasher, Hash}};

use ruwren::{send_foreign, VM, Class, ModuleLibrary, create_module, SlotType, FunctionSignature, get_slot_checked};
use sdl2::render::Texture;

use crate::{components::{Component, Transform, Sprite, Rigidbody, Animator, Tickable, ComponentBehaviour}, math::Vec2, application::App};
use uuid::Uuid;

#[macro_export]
macro_rules! mut_all {
    ($go:ident, $t:ty, $c:expr) => {
        $go.wrap_all_mut::<$t>().iter_mut().for_each($c);
    }
}
pub use mut_all;

#[macro_export]
macro_rules! find {
    ($go:ident, $t:ty, $c:expr) => {
        $go.wrap_all_mut::<$t>().iter_mut().find($c);
    }
}
pub use find;

#[macro_export]
macro_rules! tick_component {
    ($a: ty, $b: ty, $s:ident, $app: ident) => {
        if $s.has::<$a>() && $s.has::<$b>() {
            let c = $s.get::<$b>().clone();
            $s.get_mut::<$a>().tick($app.delta_time(), &c);
        }
    };
}

/// Used for identifying Gameobject
#[derive(Clone, PartialEq)]
pub struct GameObjectId {
    pub name: String,
    /// UUID generated randomly per execution. Do not rely on being the same between plays.
    pub uuid: String
}
impl GameObjectId {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            uuid: Uuid::new_v4().to_string()
        }
    }
}

pub struct GameObject {
    pub id : GameObjectId,
    pub wren_id : usize,
    pub components: Vec<Box<dyn Component>>,
    pub init: bool,
    pub start: bool,
} 

impl Clone for GameObject {
    fn clone(&self) -> Self {
        let mut g = Self { id : self.id.clone(), components: vec!(), wren_id: self.wren_id, init: self.init, start: self.start };

        for i in &self.components {
            g.components.push(i.clone_dyn());
        }

        g
    }
}

impl Class for GameObject {
    fn initialize(vm: &VM) -> GameObject {
        let id = get_slot_checked!(vm => string 1);
        GameObject::new(id)
    }
}

impl GameObject {
    pub fn new(name : String) -> Self {
        Self {
            id: GameObjectId::new(name),
            components: vec![],
            wren_id: 0,
            init: false, 
            start: false,
        }
    }

    pub fn load(&mut self, tex: &HashMap<String, Texture>) {
        if self.init {
            self.start = true;
            return;
        }

        if self.has::<Sprite>() {
            self.get_mut::<Sprite>().load(tex);
        }
        if self.has::<Rigidbody>() {
            if self.has::<Sprite>() {
                let sprite_size = self.get::<Sprite>().get_size();
                let body = self.get_mut::<Rigidbody>();  
                body.bounds = Vec2::new(sprite_size.0 as f64, sprite_size.1 as f64);
            }
            if self.has::<Transform>() {
                let p = self.get::<Transform>().position;
                self.get_mut::<Rigidbody>().position = p;
            }
        }

        self.init = true;
    }

    pub fn update(&mut self, app: &mut App) {
        tick_component!(Rigidbody, Sprite, self, app);
        tick_component!(Transform, Rigidbody, self, app);
        tick_component!(Animator, Sprite, self, app);
        tick_component!(Sprite, Animator, self, app);
    }
    
    /// Push component to GameObject with defaults
    pub fn push_component<T: 'static + Component + Default>(&mut self) {
        self.components.push(Box::new(T::default()));
    }
    
    /// Push component to GameObject with constructor
    pub fn push_component_specific<T: 'static + Component>(&mut self, c: T) {
        self.components.push(Box::new(c));
    }
    
    /// Gets Component but wraps it in Option in the case that it does not exist.
    pub fn wrap_component<T: 'static + Component>(&self) -> Option<&T> {
        for comp in &self.components {
            match comp.as_any().downcast_ref::<T>() {
                Some(val) => return Some(val),
                None => continue
            };
        }
        None
    }

    /// Checks if Component exist.
    pub fn has<T: 'static + Component>(&self) -> bool {
        match self.wrap_component::<T>() {
            Some(_) => true,
            None => false
        }
    }

    /// Gets Component but will panic if it does not exist.
    pub fn get<T: 'static + Component>(&self) -> &T {
        let mut ret = None;
        for comp in &self.components {
            match comp.as_any().downcast_ref::<T>() {
                Some(val) => {
                    ret = Some(val);
                    break;
                }
                None => continue
            };
        }
        return ret.unwrap();
    }

    /// Gets Mutable Component but will panic if it does not exist. 
    pub fn get_mut<T: 'static + Component>(&mut self) -> &mut T {
        let mut ret = None;
        for comp in &mut self.components {
            match comp.as_any_mut().downcast_mut::<T>() {
                Some(val) => {
                    ret = Some(val);
                    break;
                }
                None => continue
            };
        }
        return ret.unwrap();
    }

    /// Gets Mutable Component but wraps it in Option in the case that it does not exist.
    pub fn wrap_component_mut<T: 'static + Component>(&mut self) -> Option<&mut T> {
        for comp in &mut self.components {
            match comp.as_any_mut().downcast_mut::<T>() {
                Some(val) => return Some(val),
                None => continue
            };
        }
        None
    }
    
    /// Gets all Components of Type T 
    pub fn wrap_all<T: 'static + Component>(& self) -> Vec<& T> {
        let mut comps = vec![];
        for comp in &self.components {
            comps.push ( match comp.as_any().downcast_ref::<T>() {
                Some(val) => val,
                None => continue
            });
        }
        comps
    }
    
    pub fn wrap_all_mut<T: 'static + Component>(&mut self) -> Vec<&mut T> {
        let mut comps = vec![];
        for comp in &mut self.components {
            comps.push ( match comp.as_any_mut().downcast_mut::<T>() {
                Some(val) => val,
                None => continue
            });
        }
        comps
    }
    
    //builder
    pub fn with<T: 'static + Component + Default>(mut self) -> GameObject {
        self.components.push(Box::new(T::default()));
        self
    }
    
    pub fn with_specific<T: 'static + Component + Default>(mut self, c: T) -> GameObject {
        self.components.push(Box::new(c));
        self
    }
    
    pub fn build(mut self) -> GameObject {
        if self.has::<Rigidbody>() && self.has::<Transform>() {
            let transform = *self.get::<Transform>();
            let body = self.get_mut::<Rigidbody>();
            
            body.position = transform.position;
        }

        self
    }
    //builder end

    //wren stuff
    pub fn wren_getter_id(&self, vm: &VM) {
        vm.set_slot_new_map(0);
        vm.set_slot_string(1, "name");
        vm.set_slot_string(2, self.id.name.clone());
        vm.set_map_value(0, 1, 2);
        vm.set_slot_string(1, "uuid");
        vm.set_slot_string(2, self.id.uuid.clone());
        vm.set_map_value(0, 1, 2);
    }

    pub fn wren_setter_name(&mut self, vm: &VM) {
        let new_name = get_slot_checked!(vm => string 1);
        self.id.name = new_name;
    }

    pub fn wren_getter_name(&mut self, vm: &VM) {
        vm.set_slot_string(0, self.id.name.clone());    
    }

    pub fn wren_getter_uuid(&mut self, vm: &VM) {
        vm.set_slot_string(0, self.id.uuid.clone());    
    }

    pub fn send_to_wren(&self, slot : usize, vm : &VM) {
        send_foreign!(vm, "engine", "GameObject", self.clone() => slot);
    }

    pub fn wren_add_component(&mut self, vm : &VM) {
        let c = get_slot_checked!(vm => foreign Box<dyn Component> => 1);
        self.components.push(c.clone_dyn());
    }

    pub fn wren_get_component(&self, vm : &VM) {
        let c = vm.get_slot_string(1).unwrap();
        for i in self.components.iter().enumerate() {
            match c.as_str() {
                "Transform" => {
                    if let Some(b) = i.1.as_any().downcast_ref::<Transform>() {
                        b.send_to_wren(0, vm);
                        return;
                    }
                }
                "Sprite" => {
                    if let Some(b) = i.1.as_any().downcast_ref::<Sprite>() {
                        b.send_to_wren(0, vm);
                        return;
                    }
                }
                "Rigidbody" => {
                    if let Some(b) = i.1.as_any().downcast_ref::<Rigidbody>() {
                        b.send_to_wren(0, vm);
                        return;
                    }
                }
                "Animator" => {
                    if let Some(b) = i.1.as_any().downcast_ref::<Animator>() {
                        b.send_to_wren(0, vm);
                        return;
                    }
                }
                "ComponentBehaviour" => {
                    if let Some(b) = i.1.as_any().downcast_ref::<ComponentBehaviour>() {
                        b.send_to_wren(0, vm);
                        return;
                    }
                }
                _ => { 
                    vm.set_slot_null(0);
                }
            }
        }
    }

    pub fn wren_set_component(&mut self, vm : &VM) {
        let component_str = vm.get_slot_string(1).unwrap();

        for i in 0..self.components.len() {
            match component_str.as_str() {
                "Transform" => {
                    if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                        if let (Some(a), Some(b)) = 
                        (self.components[i].as_any_mut().downcast_mut::<Transform>(),
                        comp.as_any().downcast_ref::<Transform>()) {
                            self.components[i] = b.clone_dyn();
                            return;
                        }
                    }
                }
                "Sprite" => {
                    if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                        if let (Some(a), Some(b)) = 
                        (self.components[i].as_any_mut().downcast_mut::<Sprite>(),
                        comp.as_any().downcast_ref::<Sprite>()) {
                            self.components[i] = b.clone_dyn();
                            return;
                        }
                    }
                }
                "Rigidbody" => {
                    if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                        if let (Some(a), Some(b)) = 
                        (self.components[i].as_any_mut().downcast_mut::<Rigidbody>(),
                        comp.as_any().downcast_ref::<Rigidbody>()) {
                            self.components[i] = b.clone_dyn();
                            return;
                        }
                    }
                }
                "Animator" => {
                    if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                        if let (Some(a), Some(b)) = 
                        (self.components[i].as_any_mut().downcast_mut::<Animator>(),
                        comp.as_any().downcast_ref::<Animator>()) {
                            self.components[i] = b.clone_dyn();
                            return;
                        }
                    }
                }
                "ComponentBehaviour" => {
                    if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                        if let (Some(a), Some(b)) = 
                        (self.components[i].as_any_mut().downcast_mut::<ComponentBehaviour>(),
                        comp.as_any().downcast_ref::<ComponentBehaviour>()) {
                            self.components[i] = b.clone_dyn();
                            return;
                        }
                    }
                }
                _ => { }
            }
        }
    }
}