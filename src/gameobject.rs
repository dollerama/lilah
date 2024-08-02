use crate::{
    application::App,
    components::{
        Animator, Component, ComponentBehaviour, Rigidbody, Scene, SceneData, Sfx, Sprite, Text,
        Tickable, Transform,
    },
    math::Vec2,
    renderer::LilahTexture,
    world::StateUpdateContainer,
    LilahNotFoundError, LilahTypeError, LilahTypePanic,
};
use rusttype::Font;
use ruwren::{send_foreign, Class, VM};
use sdl2::mixer::Chunk;
use std::collections::HashMap;
use uuid::Uuid;

#[macro_export]
macro_rules! mut_all {
    ($go:ident, $t:ty, $c:expr) => {
        $go.wrap_all_mut::<$t>().iter_mut().for_each($c);
    };
}
pub use mut_all;

#[macro_export]
macro_rules! find {
    ($go:ident, $t:ty, $c:expr) => {
        $go.wrap_all_mut::<$t>().iter_mut().find($c);
    };
}
pub use find;

#[macro_export]
macro_rules! tick_component {
    ($a: ty, $b: ty, $s:ident, $app: ident) => {
        // if $s.has::<$a>() && $s.has::<$b>() {
        //     let c = $s.get::<$b>().clone();
        //     $s.get_mut::<$a>().tick($app.delta_time(), &c);
        // }
        if $s.has::<$b>() {
            let c = $s.get::<$b>().clone();
            if let Some(aa) = $s.wrap_component_mut::<$a>() {
                aa.tick($app.delta_time(), &c);
            }
        }
    };
}

/// Used for identifying Gameobject
#[derive(Clone, PartialEq)]
pub struct GameObjectId {
    pub name: String,
    /// UUID generated randomly per execution. Do not rely on being the same between plays.
    pub uuid: String,
}
impl GameObjectId {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            uuid: Uuid::new_v4().to_string(),
        }
    }
}

pub struct GameObject {
    pub id: GameObjectId,
    pub wren_id: usize,
    pub components: Vec<Box<dyn Component>>,
    pub init: bool,
    pub start: bool,
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        let mut g = Self {
            id: self.id.clone(),
            components: vec![],
            wren_id: self.wren_id,
            init: self.init,
            start: self.start,
        };

        for i in &self.components {
            g.components.push(i.clone_dyn());
        }

        g
    }
}

impl Class for GameObject {
    fn initialize(vm: &VM) -> GameObject {
        if let Some(id) = vm.get_slot_string(1) {
            GameObject::new(id)
        } else {
            LilahTypePanic!(GameObject, 1, String);
        }
    }
}

impl GameObject {
    pub fn new(name: String) -> Self {
        Self {
            id: GameObjectId::new(name),
            components: vec![],
            wren_id: 0,
            init: false,
            start: false,
        }
    }

    pub fn load(
        &mut self,
        app: &mut App,
        tex: &HashMap<String, LilahTexture>,
        fonts: &HashMap<String, Font>,
        sfx: &HashMap<String, Chunk>,
        scenes: &HashMap<String, SceneData>,
    ) -> StateUpdateContainer {
        let mut state_updates = StateUpdateContainer {
            textures: None,
            sfx: None,
        };
        if self.has::<Text>() {
            state_updates.textures = self.get_mut::<Text>().load(app, fonts).textures;
        }

        let mut sfx_updates = vec![];

        for c in self.wrap_all_mut::<Sfx>() {
            if c.play_state {
                if let Some(chunk) = sfx.get(&c.file) {
                    sfx_updates.push((c.file.clone(), c.volume as i32));
                    match sdl2::mixer::Channel::all().play(chunk, 0) {
                        Ok(ch) => {
                            c.channel = Some(ch);
                            ch.expire(-1);
                            c.play_state = false;
                        }
                        Err(e) => {
                            eprintln!("Sfx error: {}", e)
                        }
                    }
                }
            }
        }

        state_updates.sfx = Some(sfx_updates);

        if self.init {
            self.start = true;
            return state_updates;
        }

        if self.has::<Scene>() {
            self.get_mut::<Scene>().load(app, tex, scenes);
        }
        if self.has::<Sprite>() {
            self.get_mut::<Sprite>().load(app, tex);
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
        state_updates
    }

    pub fn update(&mut self, app: &mut App) {
        tick_component!(Rigidbody, Sprite, self, app);
        tick_component!(Rigidbody, Transform, self, app);
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
                None => continue,
            };
        }
        None
    }

    /// Checks if Component exist.
    pub fn has<T: 'static + Component>(&self) -> bool {
        match self.wrap_component::<T>() {
            Some(_) => true,
            None => false,
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
                None => continue,
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
                None => continue,
            };
        }
        return ret.unwrap();
    }

    /// Gets Mutable Component but wraps it in Option in the case that it does not exist.
    pub fn wrap_component_mut<T: 'static + Component>(&mut self) -> Option<&mut T> {
        for comp in &mut self.components {
            match comp.as_any_mut().downcast_mut::<T>() {
                Some(val) => return Some(val),
                None => continue,
            };
        }
        None
    }

    /// Gets all Components of Type T
    pub fn wrap_all<T: 'static + Component>(&self) -> Vec<&T> {
        let mut comps = vec![];
        for comp in &self.components {
            comps.push(match comp.as_any().downcast_ref::<T>() {
                Some(val) => val,
                None => continue,
            });
        }
        comps
    }

    pub fn wrap_all_mut<T: 'static + Component>(&mut self) -> Vec<&mut T> {
        let mut comps = vec![];
        for comp in &mut self.components {
            comps.push(match comp.as_any_mut().downcast_mut::<T>() {
                Some(val) => val,
                None => continue,
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
        if let Some(new_name) = vm.get_slot_string(1) {
            self.id.name = new_name;
        } else {
            LilahTypeError!(GameObject, 1, String);
        }
    }

    pub fn wren_getter_name(&mut self, vm: &VM) {
        vm.set_slot_string(0, self.id.name.clone());
    }

    pub fn wren_getter_uuid(&mut self, vm: &VM) {
        vm.set_slot_string(0, self.id.uuid.clone());
    }

    pub fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "GameObject", self.clone() => slot);
    }

    pub fn wren_add_component(&mut self, vm: &VM) {
        if let Some(c) = vm.get_slot_foreign::<Box<dyn Component>>(1) {
            self.components.push(c.clone_dyn());
        } else if let Some(c) = vm.get_slot_foreign::<Transform>(1) {
            self.components.push(Box::new(c.clone()));
        } else if let Some(c) = vm.get_slot_foreign::<Rigidbody>(1) {
            self.components.push(Box::new(c.clone()));
        } else if let Some(c) = vm.get_slot_foreign::<Sprite>(1) {
            self.components.push(Box::new(c.clone()));
        } else if let Some(c) = vm.get_slot_foreign::<Text>(1) {
            self.components.push(Box::new(c.clone()));
        } else if let Some(c) = vm.get_slot_foreign::<Animator>(1) {
            self.components.push(Box::new(c.clone()));
        } else if let Some(c) = vm.get_slot_foreign::<Sfx>(1) {
            self.components.push(Box::new(c.clone()));
        } else if let Some(c) = vm.get_slot_foreign::<ComponentBehaviour>(1) {
            self.components.push(Box::new(c.clone()));
        } else if let Some(c) = vm.get_slot_foreign::<Scene>(1) {
            self.components.push(Box::new(c.clone()));
        } else {
            LilahTypeError!(GameObject, 1, Component);
        }
    }

    pub fn wren_get_component(&self, vm: &VM) {
        let mut finding = String::from("");
        if let Some(c) = vm.get_slot_string(1) {
            finding = c.clone();
            for i in self.components.iter().enumerate() {
                match c.as_str() {
                    "Transform" => {
                        if let Some(b) = i.1.as_any().downcast_ref::<Transform>() {
                            b.send_to_wren(0, vm);
                            return;
                        }
                    }
                    "Scene" => {
                        if let Some(b) = i.1.as_any().downcast_ref::<Scene>() {
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
                    "Sfx" => {
                        if let Some(b) = i.1.as_any().downcast_ref::<Sfx>() {
                            b.send_to_wren(0, vm);
                            return;
                        }
                    }
                    _ => {}
                }
            }
        } else {
            LilahTypeError!(GameObject, 1, String);
        }
        LilahNotFoundError!(GameObject, Component, finding);
        vm.set_slot_null(0);
    }

    pub fn wren_get_components(&self, vm: &VM) {
        vm.set_slot_new_list(0);
        let mut list_index = -1;
        for i in self.components.iter().enumerate() {
            if let Some(b) = i.1.as_any().downcast_ref::<Transform>() {
                list_index += 1;
                b.send_to_wren(1, vm);
                vm.insert_in_list(0, list_index, 1);
            }
            if let Some(b) = i.1.as_any().downcast_ref::<Scene>() {
                list_index += 1;
                b.send_to_wren(1, vm);
                vm.insert_in_list(0, list_index, 1);
            }
            if let Some(b) = i.1.as_any().downcast_ref::<Sprite>() {
                list_index += 1;
                b.send_to_wren(1, vm);
                vm.insert_in_list(0, list_index, 1);
            }
            if let Some(b) = i.1.as_any().downcast_ref::<Rigidbody>() {
                list_index += 1;
                b.send_to_wren(1, vm);
                vm.insert_in_list(0, list_index, 1);
            }
            if let Some(b) = i.1.as_any().downcast_ref::<Animator>() {
                list_index += 1;
                b.send_to_wren(1, vm);
                vm.insert_in_list(0, list_index, 1);
            }
            if let Some(b) = i.1.as_any().downcast_ref::<ComponentBehaviour>() {
                list_index += 1;
                b.send_to_wren(1, vm);
                vm.insert_in_list(0, list_index, 1);
            }
            if let Some(b) = i.1.as_any().downcast_ref::<Sfx>() {
                list_index += 1;
                b.send_to_wren(1, vm);
                vm.insert_in_list(0, list_index, 1);
            }
        }

        if list_index == -1 {
            vm.set_slot_null(0)
        }
    }

    pub fn wren_set_component(&mut self, vm: &VM) {
        let mut finding = String::from("");
        if let Some(component_str) = vm.get_slot_string(1) {
            finding = component_str.clone();
            for i in 0..self.components.len() {
                match component_str.as_str() {
                    "Transform" => {
                        if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                            if let (Some(_a), Some(b)) = (
                                self.components[i].as_any_mut().downcast_mut::<Transform>(),
                                comp.as_any().downcast_ref::<Transform>(),
                            ) {
                                self.components[i] = b.clone_dyn();
                                return;
                            }
                        } else {
                            LilahTypeError!(GameObject, 2, Component);
                            return;
                        }
                    }
                    "Scene" => {
                        if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                            if let (Some(_a), Some(b)) = (
                                self.components[i].as_any_mut().downcast_mut::<Scene>(),
                                comp.as_any().downcast_ref::<Scene>(),
                            ) {
                                self.components[i] = b.clone_dyn();
                                return;
                            }
                        } else {
                            LilahTypeError!(GameObject, 2, Component);
                            return;
                        }
                    }
                    "Sprite" => {
                        if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                            if let (Some(_a), Some(b)) = (
                                self.components[i].as_any_mut().downcast_mut::<Sprite>(),
                                comp.as_any().downcast_ref::<Sprite>(),
                            ) {
                                self.components[i] = b.clone_dyn();
                                return;
                            }
                        } else {
                            LilahTypeError!(GameObject, 2, Component);
                            return;
                        }
                    }
                    "Rigidbody" => {
                        if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                            if let (Some(_a), Some(b)) = (
                                self.components[i].as_any_mut().downcast_mut::<Rigidbody>(),
                                comp.as_any().downcast_ref::<Rigidbody>(),
                            ) {
                                self.components[i] = b.clone_dyn();
                                return;
                            }
                        } else {
                            LilahTypeError!(GameObject, 2, Component);
                            return;
                        }
                    }
                    "Animator" => {
                        if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                            if let (Some(_a), Some(b)) = (
                                self.components[i].as_any_mut().downcast_mut::<Animator>(),
                                comp.as_any().downcast_ref::<Animator>(),
                            ) {
                                self.components[i] = b.clone_dyn();
                                return;
                            }
                        } else {
                            LilahTypeError!(GameObject, 2, Component);
                            return;
                        }
                    }
                    "ComponentBehaviour" => {
                        if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                            if let (Some(_a), Some(b)) = (
                                self.components[i]
                                    .as_any_mut()
                                    .downcast_mut::<ComponentBehaviour>(),
                                comp.as_any().downcast_ref::<ComponentBehaviour>(),
                            ) {
                                self.components[i] = b.clone_dyn();
                                return;
                            }
                        } else {
                            LilahTypeError!(GameObject, 2, Component);
                            return;
                        }
                    }
                    "Sfx" => {
                        if let Some(comp) = vm.get_slot_foreign::<Box<dyn Component>>(2) {
                            if let (Some(_a), Some(b)) = (
                                self.components[i].as_any_mut().downcast_mut::<Sfx>(),
                                comp.as_any().downcast_ref::<Sfx>(),
                            ) {
                                self.components[i] = b.clone_dyn();
                                return;
                            }
                        } else {
                            LilahTypeError!(GameObject, 2, Component);
                            return;
                        }
                    }
                    _ => {}
                }
            }
        } else {
            LilahTypeError!(GameObject, 1, String);
        }

        LilahNotFoundError!(GameObject, Component, finding);
    }
}
