use std::collections::HashMap;
use debug_print::{debug_print, debug_println, debug_eprintln};
use crate::{application::{App, Scripting}, components::{Rigidbody, Sprite, Transform}, gameobject::GameObjectId};
use crate::gameobject::GameObject;
use ruwren::{VM, create_module, Class, get_slot_checked, ModuleLibrary, FunctionSignature, VMWrapper};
use sdl2::{render::Texture, image::LoadTexture};

#[macro_export]
macro_rules! load_texture {
    ($path: expr, $state:ident, $app:ident) => {
        $state.load_texture_bytes($path, include_bytes!($path), $app);
    };
}
pub use load_texture;

pub struct WorldState {
    pub gameobjects: HashMap<String, GameObject>,
    pub textures : HashMap<String, Texture>
}

impl WorldState {
    pub fn wrap(&self, key: &str) -> Option<&GameObject> {
        if let Some(g) = self.gameobjects.iter().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            Some(g.1)
        }
        else {
            None
        }
    }

    pub fn wrap_mut(&mut self, key: &str) -> Option<&mut GameObject> {
        if let Some(g) = self.gameobjects.iter_mut().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            Some(g.1)
        }
        else {
            None
        }
    }

    pub fn get(&self, key: &str) -> &GameObject {
        if let Some(g) = self.gameobjects.iter().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            g.1
        }
        else {
            panic!("tried to get gameobject got None")
        }
    }

    pub fn get_mut(&mut self, key: &str) -> &mut GameObject {
        if let Some(g) = self.gameobjects.iter_mut().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            g.1
        }
        else {
            panic!("tried to get gameobject got None")
        }
    }

    pub fn insert_wren(&mut self, g : GameObject) {
        let mut g2 = g.clone();
        self.gameobjects.insert(g2.id.uuid.clone(), g2);
    }

    pub fn insert(&mut self, g : GameObject) {
        let mut g2 = g.clone();
        g2.wren_id = self.gameobjects.keys().len();
        self.gameobjects.insert(g2.id.uuid.clone(), g2);
    }

    pub fn load_texture(&mut self, file : &str, app : &App) {
        match app.tex_creator.load_texture(file) {
            Ok(v) => {
                self.textures.insert(file.to_string(), v);
            }
            Err(e) => {
                debug_eprintln!("Texture Error: {}", e);
            }
        };
    }

    pub fn load_texture_bytes(&mut self, name: &str, source : &[u8], app : &App) {
        match app.tex_creator.load_texture_bytes(source) {
            Ok(v) => {
                self.textures.insert(name.to_string(), v);
            }
            Err(e) => {
                debug_eprintln!("Texture Error: {}", e);
            }
        };
    }
}

pub struct World {
    pub state : WorldState,
    pub setup_callback : Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>,
    pub start_callback : Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>,
    pub update_callback : Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>
}

impl World {
    //builder
    pub fn new() -> Self {
        Self {
            state : WorldState {
                gameobjects: HashMap::new(),
                textures : HashMap::new()
            },
            setup_callback  : None,
            start_callback : None,
            update_callback : None
        }
    }

    pub fn setup(mut self, s : Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World {
        self.setup_callback  = Some(s);
        self
    }

    pub fn start(mut self, s : Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World {
        self.start_callback  = Some(s);
        self
    }
    
    pub fn tick(mut self, s : Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World {
        self.update_callback  = Some(s);
        self
    }

    pub fn run(mut self, app : &mut App, scripting : &mut Scripting) -> World {
        if self.setup_callback.is_some() {
            self.setup_callback.as_mut().unwrap()(app, &mut self.state, scripting);
        }

        for (_, i) in &mut self.state.gameobjects {
            i.load(&self.state.textures);   
        }

        if self.start_callback.is_some() {
            self.start_callback.as_ref().unwrap()(app, &mut self.state, scripting);
        }

        scripting.receive_state(app, &mut self.state);
        scripting.send_state(app, &mut self.state);

        'running: loop {
            if app.pre_frame() {
                break 'running;
            }

            scripting.tick(app, &mut self.state);
            if self.update_callback.is_some() {
                self.update_callback.as_mut().unwrap()(app, &mut self.state, scripting);
            }
            self.update(app);
            scripting.send_state(app, &mut self.state);

            self.draw(app);
            app.present_frame();
        }

        self
    }

    fn update_go(&mut self, mut app: &mut App) {
        for (_, i) in &mut self.state.gameobjects {
            i.load(&self.state.textures);   
            i.update(&mut app);
        }
    }

    pub fn draw(&self, app: &mut App) {
        for (_, i) in &self.state.gameobjects {
            if i.has::<Sprite>() && i.has::<Transform>() {
                i.get::<Sprite>().draw(app, &self.state.textures, i.get::<Transform>());
            }
        }
    }

    pub fn wrap(&self, key: &str) -> Option<&GameObject> {
        self.state.wrap(key)
    }

    pub fn wrap_mut(&mut self, key: &str) -> Option<&mut GameObject> {
        self.state.wrap_mut(key)
    }

    pub fn get(&self, key: &str) -> &GameObject {
        self.state.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> &mut GameObject {
        self.state.get_mut(key)
    }

    pub fn update(&mut self, mut app: &mut App) {
        let mut check_1 = false;

        self.update_vel_x();
        let mut collisions = Vec::<(GameObjectId, GameObjectId, bool)>::new();
        self.check_collision(&mut collisions);

        for coll in &collisions {
            if coll.2 {
                if self.get(&coll.0.uuid).init && self.get(&coll.1.uuid).init {
                    let g2_is_solid = self.get(&coll.1.uuid).get::<Rigidbody>().solid;
                    if self.get(&coll.0.uuid).has::<Rigidbody>() {
                        let body = self.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                        body.colliding = Some(coll.1.clone());
                        check_1 = true;
                        
                        if g2_is_solid {
                            body.update_correct_x();
                        }
                    }  
                }
            }
            else {
               if self.get(&coll.0.uuid).has::<Rigidbody>() {
                    let body = self.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                    body.colliding = None;
               } 
            }
        }

        self.update_vel_y();
        collisions = Vec::<(GameObjectId, GameObjectId, bool)>::new();
        self.check_collision(&mut collisions);

        for coll in &collisions {
            if coll.2 {
                if self.get(&coll.0.uuid).init && self.get(&coll.1.uuid).init {
                    let g2_is_solid = self.get(&coll.1.uuid).get::<Rigidbody>().solid;
                    if self.get(&coll.0.uuid).has::<Rigidbody>() {
                        let body = self.state.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                        body.colliding = Some(coll.1.clone());
                        if g2_is_solid {
                            body.update_correct_y();
                        }
                    } 
                }
            }
            else {
                if !check_1 {
                    if self.get(&coll.0.uuid).has::<Rigidbody>() {
                        let body = self.state.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                        body.colliding = None;
                    } 
                }
            }
        }

        self.update_go(&mut app);
    }

    fn check_collision(&self, coll: &mut Vec<(GameObjectId, GameObjectId, bool)>) {
        let mut others = false;
        for (k, i) in &self.state.gameobjects {
            for (k2, j) in &self.state.gameobjects {
                if k != k2 {
                    others = true;
                    
                    if i.has::<Rigidbody>() {
                        if j.has::<Rigidbody>() {
                            let check =  i.get::<Rigidbody>().check_collision(&j.get::<Rigidbody>());
                            coll.push((i.id.clone(), j.id.clone(), check));
                        }
                    }
                }
            }
            if !others {
                coll.push((i.id.clone(), i.id.clone(), false));
            }
        }
    }

    fn update_vel_x(&mut self) {
        for (_, i) in &mut self.state.gameobjects {
            if i.has::<Rigidbody>() {
                let body = i.get_mut::<Rigidbody>();
                body.update_vel_x();
            }
        }
    }

    fn update_vel_y(&mut self) {
        for (_, i) in &mut self.state.gameobjects {
            if i.has::<Rigidbody>() {
                let body = i.get_mut::<Rigidbody>();
                body.update_vel_y();
            }
        }
    }
}