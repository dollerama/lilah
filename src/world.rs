use crate::components::{Debug, Scene};
use crate::gameobject::GameObject;
use crate::renderer::Color;
use crate::{
    application::{App, Scripting},
    components::{Rigidbody, SceneData, Sprite, Text, Transform},
    gameobject::GameObjectId,
    math::Vec2,
    renderer::LilahTexture,
    LilahError, LilahPanic,
};
use data2sound::decode_bytes;
use debug_print::debug_println;
use glam::{Mat4, Quat, Vec3};
use image::Rgba;
use rusttype::Font;
use serde_json;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{collections::HashMap, path::Path};

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
        $state.fonts.insert(
            $path.to_string(),
            rusttype::Font::try_from_bytes(include_bytes!($path)).unwrap(),
        );
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

#[macro_export]
macro_rules! load_scene_data {
    ($path: expr, $state:ident) => {
        $state.load_scene_data($path, $path);
    };
}
pub use load_scene_data;

#[macro_export]
macro_rules! embed_scene_data {
    ($path: expr, $state:ident) => {
        $state.embed_scene_data($path, include_bytes!($path));
    };
}
pub use embed_scene_data;

pub struct StateUpdateContainer {
    pub textures: Option<(String, image::ImageBuffer<Rgba<u8>, Vec<u8>>)>,
    pub sfx: Option<Vec<(String, i32)>>,
}

pub struct WorldState<'a> {
    pub gameobjects: HashMap<String, GameObject>,
    pub textures: HashMap<String, LilahTexture>,
    pub fonts: HashMap<String, Font<'a>>,
    pub music: HashMap<String, sdl2::mixer::Music<'a>>,
    pub sfx: HashMap<String, sdl2::mixer::Chunk>,
    pub scenes: HashMap<String, SceneData>,
}

impl<'a> WorldState<'a> {
    pub fn wrap(&self, key: &str) -> Option<&GameObject> {
        if let Some(g) = self
            .gameobjects
            .iter()
            .find(|(_, k)| (k.id.name == key || k.id.uuid == key))
        {
            Some(g.1)
        } else {
            LilahError!(World, format!("Tried to get gameobject->{} got None", key));
            None
        }
    }

    pub fn wrap_mut(&mut self, key: &str) -> Option<&mut GameObject> {
        if let Some(g) = self
            .gameobjects
            .iter_mut()
            .find(|(_, k)| (k.id.name == key || k.id.uuid == key))
        {
            Some(g.1)
        } else {
            LilahError!(World, format!("Tried to get gameobject->{} got None", key));
            None
        }
    }

    pub fn get(&self, key: &str) -> &GameObject {
        if let Some(g) = self
            .gameobjects
            .iter()
            .find(|(_, k)| (k.id.name == key || k.id.uuid == key))
        {
            g.1
        } else {
            LilahPanic!(World, format!("Tried to get gameobject->{} got None", key))
        }
    }

    pub fn get_mut(&mut self, key: &str) -> &mut GameObject {
        if let Some(g) = self
            .gameobjects
            .iter_mut()
            .find(|(_, k)| (k.id.name == key || k.id.uuid == key))
        {
            g.1
        } else {
            LilahPanic!(World, format!("Tried to get gameobject->{} got None", key))
        }
    }

    pub fn insert_wren(&mut self, g: GameObject) {
        let g2 = g.clone();
        self.gameobjects.insert(g2.id.uuid.clone(), g2);
    }

    pub fn insert(&mut self, g: GameObject) {
        let mut g2 = g.clone();
        g2.wren_id = self.gameobjects.keys().len();
        self.gameobjects.insert(g2.id.uuid.clone(), g2);
    }

    pub fn load_texture(&mut self, file: &str, _app: &App) {
        let mut new_texture = unsafe { LilahTexture::new() };

        unsafe {
            new_texture.set_wrapping(gl::REPEAT);
            new_texture.set_filtering(gl::NEAREST);
        }

        unsafe {
            if let Err(e) = new_texture.load(&Path::new(file)) {
                LilahPanic!(LilahTexture, format!("Tried to load texture->{}", e))
            }
        }

        self.textures.insert(file.to_string(), new_texture);
        debug_println!("Texture loaded: {}", file);
    }

    pub fn load_texture_bytes(&mut self, name: &str, source: &[u8], _app: &App) {
        let mut new_texture = unsafe { LilahTexture::new() };
        unsafe {
            if let Err(e) = new_texture.load_as_bytes(source) {
                LilahPanic!(LilahTexture, e);
            }
        }

        unsafe {
            new_texture.set_wrapping(gl::REPEAT);
            new_texture.set_filtering(gl::NEAREST);
        }

        self.textures.insert(name.to_string(), new_texture);
        debug_println!("Texture loaded: {}", name);
    }

    pub fn load_music(&mut self, name: &str, source: &str) {
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

    pub fn load_music_bytes(&mut self, name: &str, source: &'static [u8]) {
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

    pub fn load_scene_data(&mut self, name: &str, source: &str) {
        let mut js = File::open(source).expect("file");
        let mut buf = String::from("");
        if let Err(e) = js.read_to_string(&mut buf) {
            LilahPanic!(SceneData, e);
        }

        let result: SceneData = match serde_json::from_str(buf.as_str()) {
            Ok(v) => v,
            Err(e) => {
                LilahError!(SceneData, e);
                SceneData {
                    name: "error_no_data".to_string(),
                    path: "".to_string(),
                    tile_sheets: vec![],
                    layers: vec![],
                    markers: vec![]
                }
            }
        };

        if result.name != "error_no_data" {
            self.scenes.insert(name.to_string(), result);
        }
    }

    pub fn embed_scene_data(&mut self, name: &str, source: &'static [u8]) {
        let result: SceneData = match serde_json::from_slice(source) {
            Ok(v) => v,
            Err(e) => {
                LilahError!(SceneData, e);
                SceneData {
                    name: "error_no_data".to_string(),
                    path: "".to_string(),
                    tile_sheets: vec![],
                    layers: vec![],
                    markers: vec![]
                }
            }
        };

        if result.name != "error_no_data" {
            self.scenes.insert(name.to_string(), result);
        }
    }

    pub fn load_sfx(&mut self, name: &str, source: &str) {
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

    pub fn load_sfx_bytes(&mut self, name: &str, source: &'static [u8]) {
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
    pub state: WorldState<'a>,
    pub setup_callback: Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>,
    pub start_callback: Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>,
    pub update_callback: Option<Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>>,
    sort_fudge: Vec<(String, u32)>
}

impl<'a> World<'a> {
    //builder
    pub fn new() -> Self {
        Self {
            state: WorldState {
                gameobjects: HashMap::new(),
                textures: HashMap::new(),
                fonts: HashMap::new(),
                music: HashMap::new(),
                sfx: HashMap::new(),
                scenes: HashMap::new(),
            },
            setup_callback: None,
            start_callback: None,
            update_callback: None,
            sort_fudge: vec!()
        }
    }

    pub fn setup(mut self, s: Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World<'a> {
        self.setup_callback = Some(s);
        self
    }

    pub fn start(mut self, s: Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World<'a> {
        self.start_callback = Some(s);
        self
    }

    pub fn tick(mut self, s: Box<dyn Fn(&mut App, &mut WorldState, &mut Scripting)>) -> World<'a> {
        self.update_callback = Some(s);
        self
    }

    pub fn run(mut self, app: &mut App, scripting: &mut Scripting) -> World<'a> {
        let mut camera_pos = Vec2::new(-1.0, -1000.0);
        self.state.insert(
            GameObject::new("Camera".to_string())
                .with_specific::<Transform>(Transform::new(Vec2::new(0f64, 0f64)))
                .build(),
        );
        let camera_id = self.state.wrap("Camera").unwrap().id.clone();

        if self.setup_callback.is_some() {
            self.setup_callback.as_mut().unwrap()(app, &mut self.state, scripting);
        }

        scripting.send_state(app, &mut self.state);

        for (_, i) in &mut self.state.gameobjects {
            i.load(
                app,
                &self.state.textures,
                &self.state.fonts,
                &self.state.sfx,
                &self.state.scenes,
            );
        }

        if self.start_callback.is_some() {
            self.start_callback.as_ref().unwrap()(app, &mut self.state, scripting);
        }

        scripting.receive_state(app, &mut self.state);
        scripting.send_state(app, &mut self.state);

        'running: loop {
            let frame_time = Instant::now();
            app.time.start();
            if app.pre_frame() {
                break 'running;
            }
            scripting.handle_input(app, &mut self.state);

            let camera_pos_temp = self.state.gameobjects[&camera_id.uuid]
                .get::<Transform>()
                .position;
            let camera_scale_temp = self.state.gameobjects[&camera_id.uuid]
                .get::<Transform>()
                .scale;
            let camera_rot_temp = self.state.gameobjects[&camera_id.uuid]
                .get::<Transform>()
                .rotation;
            if camera_pos != camera_pos_temp {
                camera_pos = camera_pos_temp;
                unsafe {
                    *crate::math::VIEW_MATRIX = Mat4::from_scale_rotation_translation(
                        Vec3::new(
                            camera_scale_temp.x as f32,
                            camera_scale_temp.y as f32,
                            1f32
                        ),
                        Quat::from_rotation_z(camera_rot_temp),
                        Vec3::new(
                            -camera_pos.x as f32*camera_scale_temp.x as f32,
                            -camera_pos.y as f32*camera_scale_temp.y as f32,
                            0.0,
                        )
                    );
                }
            }

            scripting.tick(app, &mut self.state);
            if self.update_callback.is_some() {
                self.update_callback.as_mut().unwrap()(app, &mut self.state, scripting);
            }
            self.update(app);

            scripting.send_state(app, &mut self.state);

            self.draw(app);
            app.present_frame();
            
            app.time.capture();
            //println!("{}", 1.0/60.0-frame_time.elapsed().as_secs_f64());
            ::std::thread::sleep(Duration::new(
                (0.01666 - frame_time.elapsed().as_secs_f64()) as u64,
                0u32,
            ));
        }

        self
    }

    fn update_go(&mut self, mut app: &mut App) {
        let mut state_updates: Vec<StateUpdateContainer> = Vec::new();
        for (_, i) in &mut self.state.gameobjects {
            state_updates.push(i.load(
                app,
                &self.state.textures,
                &self.state.fonts,
                &self.state.sfx,
                &self.state.scenes,
            ));
            i.update(&mut app);

            if let Some(spr) = i.wrap_component_mut::<Sprite>() {
                app.sort_dirty = spr.check_dirty();
            }
        }

        for su in state_updates {
            if let Some(ftu) = su.textures {
                if let Some(t) = self.state.textures.get_mut(&ftu.0) {
                    unsafe {
                        let _ = t.load_as_dyn(ftu.1);
                    }
                } else {
                    let t = unsafe {
                        let mut nt = LilahTexture::new();

                        nt.set_wrapping(gl::REPEAT);
                        nt.set_filtering(gl::NEAREST);

                        let _ = nt.load_as_dyn(ftu.1);
                        nt
                    };
                    self.state.textures.insert(ftu.0, t);
                }
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

    pub fn draw(&mut self, app: &mut App) {
        if app.sort_dirty {
            self.sort_fudge = vec!();
            for i in &self.state.gameobjects {
                if let Some(s) = i.1.wrap_component::<Scene>() {
                    for j in 0..s.tiles.len() {
                        self.sort_fudge.push((i.0.clone(), j as u32));
                    }
                } else 
                if let Some(s) = i.1.wrap_component::<Sprite>() {
                    self.sort_fudge.push((i.0.clone(), s.get_sort()));
                }
                 else if let Some(t) = i.1.wrap_component::<Text>() {
                    self.sort_fudge.push((i.0.clone(), t.get_sort()));
                }
            }
            self.sort_fudge.sort_by(|a, b| {
                a.1.cmp(&b.1)
            });
            app.sort_dirty = false;
        }

        for index in self.sort_fudge.iter() {
            let i = self.state.gameobjects.get(&index.0);
            if let Some(i) = i {
                if let Some(trans) = i.wrap_component::<Transform>() {
                    if let Some(s) = i.wrap_component::<Sprite>() {
                        s.draw(app, &self.state.textures, trans);
                    }
                    if let Some(t) = i.wrap_component::<Text>() {
                        t.draw(app, &self.state.textures, trans);
                    }
                    if let Some(s) = i.wrap_component::<Scene>() {
                        s.draw(index.1 as usize, app, &self.state.textures, trans);
                    }
                }
            }
        }
        unsafe {
            for line in crate::application::LINES.iter() {
                Debug::draw_line(line.0, line.1, line.2.clone());
            }
            crate::application::LINES.clear();
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
        let mut collisions: Vec<(GameObjectId, GameObjectId, (bool, Vec2))> =
            Vec::<(GameObjectId, GameObjectId, (bool, Vec2))>::new();
            
        self.update_vel_x(app.delta_time());
        
        self.check_collision(&mut collisions, &app);

        for coll in &collisions {
            if coll.2 .0 {
                if self.get(&coll.0.uuid).init && self.get(&coll.1.uuid).init {
                    let g2_is_solid = self.get(&coll.1.uuid).get::<Rigidbody>().solid;
                    if self.get(&coll.0.uuid).has::<Rigidbody>() {
                        let body = self.state.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                        body.colliding = Some(coll.1.clone());
                        if g2_is_solid {
                            body.position.x -= ((body.velocity * app.delta_time()) * 100 * coll.2.1.magnitude()).x;
                        }

                        let body2 = self.get_mut(&coll.1.uuid).get_mut::<Rigidbody>();
                        body2.colliding = Some(coll.0.clone());
                    }
                }
            }
        }

        self.update_vel_y(app.delta_time());
        collisions.clear();
        self.check_collision(&mut collisions, &app);

        for coll in &collisions {
            if coll.2 .0 {
                if self.get(&coll.0.uuid).init && self.get(&coll.1.uuid).init {
                    let g2_is_solid = self.get(&coll.1.uuid).get::<Rigidbody>().solid;
                    if self.get(&coll.0.uuid).has::<Rigidbody>() {
                        let body = self.state.get_mut(&coll.0.uuid).get_mut::<Rigidbody>();
                        body.colliding = Some(coll.1.clone());
                        if g2_is_solid {
                            body.position.y -= ((body.velocity * app.delta_time()) * 100 * coll.2.1.magnitude()).y;
                        }

                        let body2 = self.get_mut(&coll.1.uuid).get_mut::<Rigidbody>();
                        body2.colliding = Some(coll.0.clone());
                    }
                }
            }
        }

        self.update_go(&mut app);
    }

    fn check_collision(
        &self,
        coll: &mut Vec<(GameObjectId, GameObjectId, (bool, Vec2))>,
        app: &App,
    ) {
        let mut others = false;
        for (k, i) in &self.state.gameobjects {
            for (k2, j) in &self.state.gameobjects {
                if k != k2 {
                    others = true;

                    if let (Some(ii), Some(jj)) = (i.wrap_component::<Rigidbody>(), j.wrap_component::<Rigidbody>()) {
                        if let (None,  None) = (i.wrap_component::<Scene>(), j.wrap_component::<Scene>()) {
                            let check =
                                ii
                                .check_collision_sat(jj, app);
                            coll.push((i.id.clone(), j.id.clone(), check));
                        }
                    }

                    if let (Some(ii), Some(jj)) = (i.wrap_component::<Rigidbody>(), j.wrap_component::<Scene>()) {
                        for r in &jj.rigidbodies {
                            let check = ii.check_collision_sat(r, app);
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

    fn update_vel_x(&mut self, dt: f64) {
        for (_, i) in &mut self.state.gameobjects {
            if let Some(ii) = i.wrap_component_mut::<Rigidbody>() {
                let body = ii;
                body.colliding = None;
                body.update_vel_x(dt);
            }
        }
    }

    fn update_vel_y(&mut self, dt: f64) {
        for (_, i) in &mut self.state.gameobjects {
            if let Some(ii) = i.wrap_component_mut::<Rigidbody>() {
                let body = ii;
                body.update_vel_y(dt);
            }
        }
    }
}
