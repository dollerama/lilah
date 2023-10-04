use std::{collections::HashMap, hash::Hash};
use sdl2::{keyboard::Keycode, mouse::MouseButton};
use crate::math::Vec2;

/// Contains info needed for mappings
#[derive(Debug, Eq, Copy, Clone, Hash, PartialEq)]
pub struct InputInfo {
    pub pressed: bool,
    pub pressed_down: bool,
}

/// Contains info needed for bindings
#[derive(Debug, Eq, Copy, Clone, Hash, PartialEq)]
pub struct BindingInfo {
    /// Bind for 1 value.
    pub positive: Keycode,
    /// Bind for -1 value.
    pub negative: Keycode
}

/// Direct key mappings and abstract bindings
#[derive(Debug)]
pub struct Input {
    /// Key Mappings
    pub mappings: HashMap<Keycode, InputInfo>,
    /// Mouse Mappings
    pub mouse_mapping: HashMap<MouseButton, InputInfo>,
    /// Mouse Position
    pub mouse_pos: Vec2,
    /// Abstract Key Bindings
    pub bindings: HashMap<String, BindingInfo>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mappings: HashMap::new(),
            mouse_mapping: HashMap::new(),
            mouse_pos: Vec2::ZERO,
            bindings: HashMap::new(),
        }
    }

    pub fn update_mouse_pos(&mut self, pos : Vec2) {
        self.mouse_pos = pos;
    }

    pub fn update_mapping(&mut self, keypair: (&Keycode, &InputInfo)) {
        self.mappings.insert(*keypair.0, *keypair.1);
    }

    pub fn update_mouse(&mut self, keypair: (&MouseButton, &InputInfo)) {
        self.mouse_mapping.insert(*keypair.0, *keypair.1);
    }

    pub fn update_binding(&mut self, keypair: (&str, &BindingInfo)) {
        self.bindings.insert(keypair.0.to_string(), *keypair.1);
    }
    
    pub fn get_mapping(&mut self, key: Keycode) -> Option<&mut InputInfo> {
        self.mappings.get_mut(&key)
    }

    pub fn get_mouse_mapping(&mut self, key: MouseButton) -> Option<&mut InputInfo> {
        self.mouse_mapping.get_mut(&key)
    }

    pub fn get_binding(&mut self, key: &str) -> Option<&mut BindingInfo> {
        self.bindings.get_mut(key)
    }
    
    ///get key pressed
    pub fn get_key(&mut self, key: Keycode) -> bool {
        match  self.get_mapping(key) {
            Some(p) => p.pressed,
            None => {
                false
            }
        }
    }

    /// Get mouse pressed
    pub fn get_mouse(&mut self, key: MouseButton) -> bool {
        match  self.get_mouse_mapping(key) {
            Some(p) => p.pressed,
            None => {
                false
            }
        }
    }

    /// Get key down this frame
    pub fn get_key_down(&mut self, key: Keycode) -> bool {
        match  self.get_mapping(key) {
            Some(p) => {
                let ret = p.pressed_down;
                p.pressed_down = false;
                ret
            }
            None => {
                false
            }
        }
    }

    /// Get mouse down this frame
    pub fn get_mouse_down(&mut self, key: MouseButton) -> bool {
        match  self.get_mouse_mapping(key) {
            Some(p) => {
                let ret = p.pressed_down;
                p.pressed_down = false;
                ret
            }
            None => {
                false
            }
        }
    }
    
    /// Get Vec2 from two bindings
    pub fn bind_to_2d(&mut self, key: &str, key2: &str) -> Vec2 {
        Vec2::new(self.bind_to_axis(key) as f64, self.bind_to_axis(key2) as f64)
    }

    /// Get axis from -1,1 from binding 
    pub fn bind_to_axis(&mut self, axis: &str) -> i32 {
        let ax = match self.get_binding(axis) {
            Some(p) => {
                (p.negative, p.positive)
            },
            None => {
                return 0;
            }
        };

        let mut ret = 0;
        if self.get_key(ax.0) {
            ret -= 1;
        }
        if self.get_key(ax.1) {
            ret += 1;
        }

        ret
    }
}

#[macro_export]
macro_rules! input_keys_down {
    ($inp:ident, $($args:expr),*) => {{
        let result = true;
        $(
            let result = result && $inp.get_key_down($args);
        )*
        result
    }}
}
pub use input_keys_down;

#[macro_export]
macro_rules! input_bind {
    ($inp:ident, $str: expr, $k1: expr, $k2: expr) => {
        $inp.input.update_binding((
        $str, 
        &BindingInfo{negative: $k1, positive: $k2}));
    };
}
pub use input_bind;
