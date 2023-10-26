use std::{collections::HashMap, path::Path};
use data2sound::decode_bytes;
use debug_print::debug_println;
use crate::{application::{App, Scripting, LilahTexture}, components::{Rigidbody, Sprite, Transform, Text}, gameobject::GameObjectId, LilahError, LilahPanic, math::Vec2};
use crate::gameobject::GameObject;
use sdl2::{render::Texture, image::LoadTexture};
use image::{GenericImageView, DynamicImage};

#[macro_export]
macro_rules! embed_texture {
    ($path: expr, $state:ident, $app:ident) => {
        $state.load_texture_bytes($path, include_bytes!($path), $app);
    };
}
pub use embed_texture;

#[macro_export]
macro_rules! embed_font {
    ($path: expr, $state:ident) => {
        $state.fonts.insert($path.to_string(), include_bytes!($path).to_vec());
    };
}
pub use embed_font;

#[macro_export]
macro_rules! embed_music {
    ($path: expr, $state:ident) => {
        $state.load_music_bytes($path, include_bytes!($path));
    };
}
pub use embed_music;


#[macro_export]
macro_rules! embed_sfx {
    ($path: expr, $state:ident) => {
        $state.load_sfx_bytes($path, include_bytes!($path));
    };
}
pub use embed_sfx;

#[macro_export]
macro_rules! load_texture {
    ($path: expr, $state:ident, $app:ident) => {
        $state.load_texture($path, $app);
    };
}
pub use load_texture;

#[macro_export]
macro_rules! load_music {
    ($path: expr, $state:ident) => {
        $state.load_music($path, $path);
    };
}
pub use load_music;

#[macro_export]
macro_rules! load_sfx {
    ($path: expr, $state:ident) => {
        $state.load_sfx($path, $path);
    };
}
pub use load_sfx;

pub struct StateUpdateContainer {
    pub textures: Option<(String, LilahTexture)>,
    pub sfx: Option<Vec<(String, i32)>>
}

pub struct WorldState<'a> {
    pub gameobjects: HashMap<String, GameObject>,
    pub textures : HashMap<String, LilahTexture>,
    pub fonts : HashMap<String, Vec<u8>>,
    pub music : HashMap<String, sdl2::mixer::Music<'a>>,
    pub sfx : HashMap<String, sdl2::mixer::Chunk>
}

impl<'a> WorldState<'a> {
    pub fn wrap(&self, key: &str) -> Option<&GameObject> {
        if let Some(g) = self.gameobjects.iter().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            Some(g.1)
        }
        else {
            LilahError!(World, format!("Tried to get gameobject->{} got None", key));
            None
        }
    }

    pub fn wrap_mut(&mut self, key: &str) -> Option<&mut GameObject> {
        if let Some(g) = self.gameobjects.iter_mut().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            Some(g.1)
        }
        else {
            LilahError!(World, format!("Tried to get gameobject->{} got None", key));
            None
        }
    }

    pub fn get(&self, key: &str) -> &GameObject {
        if let Some(g) = self.gameobjects.iter().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            g.1
        }
        else {
            LilahPanic!(World, format!("Tried to get gameobject->{} got None", key))
        }
    }

    pub fn get_mut(&mut self, key: &str) -> &mut GameObject {
        if let Some(g) = self.gameobjects.iter_mut().find(|(_, k)| (k.id.name == key || k.id.uuid == key)) {
            g.1
        }
        else {
            LilahPanic!(World, format!("Tried to get gameobject->{} got None", key))
        }
    }

    pub fn insert_wren(&mut self, g : GameObject) {
        let g2 = g.clone();
        self.gameobjects.insert(g2.id.uuid.clone(), g2);
    }

    pub fn insert(&mut self, g : GameObject) {
        let mut g2 = g.clone();
        g2.wren_id = self.gameobjects.keys().len();
        self.gameobjects.insert(g2.id.uuid.clone(), g2);
    }

    pub fn load_texture(&mut self, file : &str, app : &App) {
        let new_texture = unsafe { 
            LilahTexture::new() 
        };

        unsafe {
            new_texture.set_wrapping(gl::REPEAT);
            new_texture.set_filtering(gl::LINEAR);
        }

        unsafe {
            if let Err(e) = new_texture.load(&Path::new(file)) {
                LilahPanic!(LilahTexture, format!("Tried to load texture ->{}", e))
            }
        }

        self.textures.insert(file.to_string(), new_texture);
        debug_println!("Texture loaded: {}", file);
    }

    pub fn load_texture_bytes(&mut self, name: &str, source : &[u8], app : &App) {
        let new_texture = unsafe { LilahTexture::new() };
        unsafe {
            new_texture.load_as_bytes(source);
        }

        unsafe {
            new_texture.set_wrapping(gl::REPEAT);
            new_texture.set_filtering(gl::LINEAR);
        }

        self.textures.insert(name.to_string(), new_texture);
        debug_println!("Texture loaded: {}", name);
    }

    pub fn load_music(&mut self, name: &str, source : &str) {
        match sdl2::mixer::Music::from_file(Path::new(source)) {
            Ok(music) => {
                debug_println!("Music loaded: {}", name);
                self.music.insert(name.to_string(), music);
            }
            Err(e) => {
                LilahError!(Sfx, e);
            }
        }
    }

    pub fn load_music_bytes(&mut self, name: &str, source : &'static [u8]) {
        match sdl2::mixer::Music::from_static_bytes(source) {
            Ok(music) => {
                debug_println!("Music loaded: {}", name);
                self.music.insert(name.to_string(), music);
            }
            Err(e) => {
                LilahError!(Sfx, e);
            }
        }
    }

    pub fn load_sfx(&mut self, name: &str, source : &str) {
        match sdl2::mixer::Chunk::from_file(Path::new(source)) {
            Ok(sfx) => {
                debug_println!("Sfx loaded: {}", name);
                self.sfx.insert(name.to_string(), sfx);
            }
            Err(e) => {
                LilahError!(Sfx, e);
            }
        }
    }

    pub fn load_sfx_bytes(&mut self, name: &str, source : &'static [u8]) {
        let encoded = decode_bytes(source).unwrap();
        match sdl2::mixer::Chunk::from_raw_buffer(encoded.into()) {
            Ok(sfx) => {
                debug_println!("Sfx loaded: {}", name);
                self.sfx.insert(name.to_string(), sfx);
            }
            Err(e) => {
                LilahError!(Sfx, e);
            }
        }
    }
}

pub struct World<'a> {
    pub state : WorldState<'a>,
    pub setup_callback : Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>,
    pub start_callback : Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>,
    pub update_callback : Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>
}

impl<'a> World<'a> {
    //builder
    pub fn new() -> Self {
        Self {
            state : WorldState {
                gameobjects: HashMap::new(),
                textures : HashMap::new(),
                fonts: HashMap::new(),
                music: HashMap::new(),
                sfx: HashMap::new()
            },
            setup_callback  : None,
            start_callback : None,
            update_callback : None
        }
    }

    pub fn setup(mut self, s : Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World<'a> {
        self.setup_callback  = Some(s);
        self
    }

    pub fn start(mut self, s : Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World<'a> {
        self.start_callback  = Some(s);
        self
    }
    
    pub fn tick(mut self, s : Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World<'a> {
        self.update_callback  = Some(s);
        self
    }

    pub fn run(mut self, app : &mut App, scripting : &mut Scripting) -> World<'a> {  
        self.state.insert(GameObject::new("Camera".to_string()).with::<Transform>().build());

        if self.setup_callback.is_some() {
            self.setup_callback.as_mut().unwrap()(app, &mut self.state, scripting);
        }

        scripting.send_state(app, &mut self.state);

        for (_, i) in &mut self.state.gameobjects {
            i.load(app, &self.state.textures, &self.state.fonts, &self.state.sfx);   
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
            scripting.handle_input(app, &mut self.state);

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
        let mut state_updates: Vec<StateUpdateContainer> = Vec::new();
        for (_, i) in &mut self.state.gameobjects {
            state_updates.push(i.load(app,&self.state.textures, &self.state.fonts, &self.state.sfx));   
            i.update(&mut app);
        }

        for su in state_updates {
            if let Some(ftu) = su.textures {
                self.state.textures.insert(ftu.0, ftu.1);
            }
            if let Some(stu) = su.sfx {
                for i in stu {
                    if let Some(j) = self.state.sfx.get_mut(&i.0) {
                        j.set_volume(i.1);
                    }
                }
            }
        }
    }

    pub fn draw(&self, app: &mut App) {
        let camera = self.state.wrap("Camera");
        let mut camera_pos = None;
        if let Some(cam) = camera {
            camera_pos = Some(cam.get::<Transform>().position);
        }

        for (_, i) in &self.state.gameobjects {
            if i.has::<Transform>() {
                if i.has::<Sprite>() {
                    i.get::<Sprite>().draw(app, &self.state.textures, i.get::<Transform>(), &camera_pos);
                }
                if i.has::<Text>() {
                    i.get::<Text>().draw(app, &self.state.textures, i.get::<Transform>(), &camera_pos);
                }
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
        self.update_vel_x();
        let mut collisions = Vec::<(GameObjectId, GameObjectId, (bool, Vec2))>::new();
        self.check_collision(&mut collisions);

        for coll in &collisions {
            if coll.2.0 {
                if self.get(&coll.0.uuid).start && self.get(&coll.1.uuid).start {
                    let g2_is_solid = self.get(&coll.1.uuid).get::<Rigidbody>().solid;
                    if self.get(&coll.0.uuid).has::<Rigidbody>() {
                        let body = self.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                        body.colliding = Some(coll.1.clone());
                        if g2_is_solid {
                            body.update_correct_x();
                        }

                        let body2 = self.get_mut(&coll.1.uuid).get_mut::<Rigidbody>();
                        body2.colliding = Some(coll.0.clone());
                    }  
                }
            }
        }   

        self.update_vel_y();
        collisions = Vec::<(GameObjectId, GameObjectId, (bool, Vec2))>::new();
        self.check_collision(&mut collisions);

        for coll in &collisions {
            if coll.2.0 {
                if self.get(&coll.0.uuid).init && self.get(&coll.1.uuid).init {
                    let g2_is_solid = self.get(&coll.1.uuid).get::<Rigidbody>().solid;
                    if self.get(&coll.0.uuid).has::<Rigidbody>() {
                        let body = self.state.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                        body.colliding = Some(coll.1.clone());
                        if g2_is_solid {
                            body.update_correct_y();
                        }

                        let body2 = self.get_mut(&coll.1.uuid).get_mut::<Rigidbody>();
                        body2.colliding = Some(coll.0.clone());
                    } 
                }
            }
        }

        self.update_go(&mut app);
    }

    fn check_collision(&self, coll: &mut Vec<(GameObjectId, GameObjectId, (bool, Vec2))>) {
        let mut others = false;
        for (k, i) in &self.state.gameobjects {
            for (k2, j) in &self.state.gameobjects {
                if k != k2 {
                    others = true;
                    
                    if i.has::<Rigidbody>() {
                        if j.has::<Rigidbody>() {
                            let check =  i.get::<Rigidbody>().check_collision_sat(&j.get::<Rigidbody>());
                            coll.push((i.id.clone(), j.id.clone(), check));
                        }
                    }
                }
            }
            if !others {
                coll.push((i.id.clone(), i.id.clone(), (false, Vec2::ZERO)));
            }
        }
    }

    fn update_vel_x(&mut self) {
        for (_, i) in &mut self.state.gameobjects {
            if i.has::<Rigidbody>() {
                let body = i.get_mut::<Rigidbody>();
                body.colliding = None;
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