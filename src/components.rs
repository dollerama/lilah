use crate::gameobject::GameObjectId;
use crate::math::Vec2;
use crate::renderer::{Buffer, Color, LilahTexture, Vertex, VertexArray};
use crate::world::StateUpdateContainer;
use crate::{application::App, gameobject::GameObject};
use crate::{set_attribute, LilahNotFoundError, LilahPanic, LilahTypeError, LilahTypePanic};
use gl::types::*;
use glam::{Mat4, Quat, Vec3};
use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};
use ruwren::{create_module, send_foreign, Class, ModuleLibrary, VM};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::ffi::CString;
use std::{any::Any, collections::HashMap};
use uuid::Uuid;

/// Tick/Update Component
pub trait Tickable<T: Component> {
    ///Tick Component with delta time and Component
    /// # Example
    /// impliments tickable for Transform that depends on Rigidbody. This snippet will set the transform to the rigidbody position when ticked.
    /// ```rust, ignore
    /// impl Tickable<Rigidbody> for Transform {
    ///     fn tick(&mut self, _: f32, d: &Rigidbody) {
    ///         self.position = d.position;
    ///     }
    /// }
    /// ```
    fn tick(&mut self, dt: f64, d: &T);
}

pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn send_to_wren(&self, slot: usize, vm: &VM);
    fn clone_dyn(&self) -> Box<dyn Component>;
}

/// Transform Component for GameObjects
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Transform {
    pub position: Vec2,
    pub pivot: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

/// Sfx Component for GameObjects
#[derive(Clone)]
pub struct Sfx {
    pub name: String,
    pub file: String,
    pub play_state: bool,
    pub volume: f64,
    pub channel: Option<sdl2::mixer::Channel>,
}

/// Rigidbody Component for GameObjects
#[derive(PartialEq, Clone)]
pub struct Rigidbody {
    pub position: Vec2,
    pub pivot: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    /// Bounds of Collider
    pub bounds: Vec2,
    pub velocity: Vec2,
    /// GameObjectID of current collider
    pub colliding: Option<GameObjectId>,
    /// If set to false colliding is still populated but the rigidbody will not correct its velocity when collisions are detected.
    pub solid: bool,
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
    pub texture_id: String,

    pub tint: Color,

    pub sort: u32,
    pub sort_dirty: bool,

    vertex_buffer: Option<Buffer>,
    vertex_array: Option<VertexArray>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Layer {
    #[serde_as(as = "Vec<(_, _)>")]
    pub tiles: HashMap<(i32, i32), Tile>,
    pub visible: bool,
    pub collision: bool,
    pub tile_sheet: String,
    pub current_tile_item: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tile {
    pub sheet: String,
    pub sheet_id: (u32, u32),
    pub position: (f32, f32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Marker {
    pub position: [f32; 2],
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneData {
    pub name: String,
    pub path: String,
    pub tile_sheets: Vec<TileSheet>,
    pub layers: Vec<Layer>,
    pub markers: Vec<Marker>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TileSheet {
    pub filename: String,
    pub path: String,
    pub absolute_path: String,
    pub tile_size: (u32, u32),
    pub sheet_size: (u32, u32),
}

#[derive(Clone, Default)]
pub struct Scene {
    pub file: String,
    pub tiles: Vec<Vec<Sprite>>,
    pub markers: Vec<Marker>,
    pub transforms: Vec<Vec<Transform>>,
    pub rigidbodies: Vec<Rigidbody>,
}

/// Animator Component for GameObjects
#[derive(PartialEq, Default, Clone)]
pub struct Animator {
    /// Name of State(String), sprite sheet index(i32, i32)
    states: HashMap<String, (i32, i32)>,
    current_state: String,
    pub current_frame: f64,
    pub speed: f64,
    playing: bool,
}

/// Behaviour Component for GameObjects
#[derive(Clone)]
pub struct ComponentBehaviour {
    /// Name of wren class to link to behaviour
    pub component: String,
    pub uuid: String,
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

    pub color: Color,

    sort: u32,
    sort_dirty: bool,

    vertex_buffer: Option<Buffer>,
    vertex_array: Option<VertexArray>,
}

pub struct Debug {}

//component impls
impl Sfx {
    pub fn new(name: String, file: String) -> Self {
        Self {
            name,
            file,
            play_state: false,
            volume: 128.0,
            channel: None,
        }
    }

    pub fn play(&mut self) {
        self.play_state = true;
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_name_getter(&self, vm: &VM) {
        vm.set_slot_string(0, self.name.clone());
    }

    fn wren_name_setter(&mut self, vm: &VM) {
        match vm.get_slot_string(1) {
            Some(name) => self.name = name.clone(),
            None => {
                eprintln!("Sfx Error: Arg (1) must be of type String");
            }
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
            None => {
                eprintln!("Sfx Error: Arg (1) must be of type Double");
            }
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
                } else {
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
                } else {
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
                } else {
                    eprintln!("Sfx Error: Arg (2) must be of type String");
                }
            }
            None => {
                eprintln!("Sfx Error: Arg (1) must be of type GameObject");
            }
        }
    }
}

impl Scene {
    pub fn new(file_name: String) -> Self {
        Self {
            file: file_name,
            tiles: vec![],
            transforms: vec![],
            rigidbodies: vec![],
            markers: vec![]
        }
    }

    pub fn load(
        &mut self,
        app: &mut App,
        textures: &HashMap<String, LilahTexture>,
        scenes: &HashMap<String, SceneData>,
    ) {
        let this_scene = &scenes[self.file.clone().as_str()];
        self.markers = this_scene.markers.clone();

        for layer in &this_scene.layers {
            let mut current_tiles = vec![];
            let mut current_trans = vec![];

            for tile in &layer.tiles {
                let mut current_sheet = "".to_string();
                let mut current_sheet_id = 0;
                for sheet in 0..this_scene.tile_sheets.len() {
                    if this_scene.tile_sheets[sheet].path == tile.1.sheet {
                        current_sheet = this_scene.tile_sheets[sheet].absolute_path.to_string();
                        current_sheet_id = sheet;
                        break;
                    }
                }
                let mut new_tile = Sprite::new(current_sheet.as_str());
                new_tile.cut_sprite_sheet(
                    tile.1.sheet_id.0 as i32,
                    tile.1.sheet_id.1 as i32,
                    this_scene.tile_sheets[current_sheet_id].sheet_size.0
                        / this_scene.tile_sheets[current_sheet_id].tile_size.0,
                    this_scene.tile_sheets[current_sheet_id].sheet_size.1
                        / this_scene.tile_sheets[current_sheet_id].tile_size.1,
                );
                new_tile.sort = self.tiles.len() as u32;

                current_tiles.push(new_tile);
                let new_trans = Transform::new(Vec2::new(
                    tile.1.position.0 as f64,
                    tile.1.position.1 as f64,
                ));
                current_trans.push(new_trans);
                if layer.collision {
                    let mut r = Rigidbody::new(Vec2::new(
                        tile.1.position.0 as f64,
                        tile.1.position.1 as f64,
                    ));
                    r.bounds = Vec2::new(
                        this_scene.tile_sheets[current_sheet_id].tile_size.0 as f64,
                        this_scene.tile_sheets[current_sheet_id].tile_size.1 as f64,
                    );
                    self.rigidbodies.push(r);
                }
            }

            self.tiles.push(current_tiles);
            self.transforms.push(current_trans);
        }

        for i in &mut self.tiles {
            for j in i {
                j.load(app, textures);
            }
        }
    }

    pub fn draw(&self, sort: usize, app: &mut App, textures: &HashMap<String, LilahTexture>, t: &Transform) {
        if sort > self.tiles.len() {
            return;
        }

        for j in 0..self.tiles[sort].len() {
            let trans = &self.transforms[sort][j];
            let new_trans = Transform::new(trans.position + t.position);
            self.tiles[sort][j].draw(app, textures, &new_trans);
        }
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Scene", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_markers(&self, vm: &VM) {
        vm.set_slot_new_list(0);
        for i in self.markers.iter().enumerate() {
            vm.set_slot_new_map(1);

            vm.set_slot_string(2, i.1.name.clone());
            vm.set_slot_new_foreign_scratch("math", "Vec2", Vec2::new(i.1.position[0] as f64, i.1.position[1] as f64), 3, 4);
            vm.set_map_value(1, 2, 3);

            vm.insert_in_list(0, 0 as i32, 1);
        }
    }
}

//component impls
impl Transform {
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            pivot: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }

    pub fn relative_position(&self) -> Vec2 {
        self.position
    }

    pub fn world_to_screen_position(&self, camera: &Vec2, screen_y: f64) -> Vec2 {
        Vec2::new(
            self.position.x - camera.x,
            (-self.position.y + screen_y) - camera.y,
        )
    }

    pub fn get_pivot(&self, size: &Vec2) -> Vec2 {
        Vec2::new(
            (self.pivot.x / size.x) * self.scale.x,
            (self.pivot.y / size.y) * self.scale.y,
        )
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_get_pos(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.position => 0);
    }

    fn wren_get_pivot(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.pivot => 0);
    }

    fn wren_get_scale(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.scale => 0);
    }

    fn wren_get_rotation(&self, vm: &VM) {
        vm.set_slot_double(0, self.rotation as f64);
    }

    fn wren_set_pos(&mut self, vm: &VM) {
        match vm.get_slot_foreign::<Vec2>(1) {
            Some(pos) => self.position = *pos,
            None => {
                LilahTypeError!(Transform, 1, Vec2);
            }
        }
    }

    fn wren_set_pivot(&mut self, vm: &VM) {
        match vm.get_slot_foreign::<Vec2>(1) {
            Some(pivot) => self.pivot = *pivot,
            None => {
                LilahTypeError!(Transform, 1, Vec2);
            }
        }
    }

    fn wren_set_scale(&mut self, vm: &VM) {
        match vm.get_slot_foreign::<Vec2>(1) {
            Some(scale) => self.scale = *scale,
            None => {
                LilahTypeError!(Transform, 1, Vec2);
            }
        }
    }

    fn wren_set_rotation(&mut self, vm: &VM) {
        match vm.get_slot_double(1) {
            Some(rotation) => self.rotation = rotation as f32,
            None => {
                LilahTypeError!(Transform, 1, f64);
            }
        }
    }

    fn wren_set_pos_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(pos) => comp.get_mut::<Transform>().position = *pos,
                None => {
                    LilahTypeError!(Transform, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_pivot_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(pivot) => comp.get_mut::<Transform>().pivot = *pivot,
                None => {
                    LilahTypeError!(Transform, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(pos_x) => comp.get_mut::<Transform>().position.x = pos_x,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(pos_y) => comp.get_mut::<Transform>().position.y = pos_y,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_update_pos_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(pos) => comp.get_mut::<Transform>().position += *pos,
                None => {
                    LilahTypeError!(Transform, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_update_pos_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(pos_x) => comp.get_mut::<Transform>().position.x += pos_x,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_update_pos_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(pos_y) => comp.get_mut::<Transform>().position.y += pos_y,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_scale_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(scale) => comp.get_mut::<Transform>().scale = *scale,
                None => {
                    LilahTypeError!(Transform, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_scale_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(scale_x) => comp.get_mut::<Transform>().scale.x = scale_x,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_scale_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(scale_y) => comp.get_mut::<Transform>().scale.y = scale_y,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_update_scale_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(scale) => comp.get_mut::<Transform>().scale += *scale,
                None => {
                    LilahTypeError!(Transform, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_update_scale_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(scale_x) => comp.get_mut::<Transform>().scale.x += scale_x,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_update_scale_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(scale_y) => comp.get_mut::<Transform>().scale.y += scale_y,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_set_rot_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(rotation) => comp.get_mut::<Transform>().rotation = rotation as f32,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }

    fn wren_update_rot_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(rotation) => comp.get_mut::<Transform>().rotation += rotation as f32,
                None => {
                    LilahTypeError!(Transform, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Transform, 1, GameObject);
            }
        }
    }
    //for wren
}

impl Rigidbody {
    pub fn new(pos: Vec2) -> Self {
        Self {
            bounds: Vec2::ONE,
            pivot: Vec2::ZERO,
            velocity: Vec2::ZERO,
            rotation: 0.0,
            position: pos,
            scale: Vec2::ONE,
            colliding: None,
            solid: true,
        }
    }

    pub fn new_without_pos() -> Self {
        Self {
            bounds: Vec2::ONE,
            pivot: Vec2::ZERO,
            velocity: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
            position: Vec2::ZERO,
            colliding: None,
            solid: true,
        }
    }

    pub fn update_vel_y(&mut self, dt: f64) {
        self.position.y += self.velocity.y * dt;
    }

    pub fn update_vel_x(&mut self, dt: f64) {
        self.position.x += self.velocity.x * dt;
    }

    pub fn update_correct_y(&mut self, dt: f64) {
        self.position.y -= self.velocity.y  * dt;
    }

    pub fn update_correct_x(&mut self, dt: f64) {
        self.position.x -= self.velocity.x * dt;
    }

    pub fn check_collision_sat(&self, other: &Rigidbody, app: &App) -> (bool, Vec2) {
        let r1 = crate::math::Rect::new_from_rigidbody(self, app);
        let r2 = crate::math::Rect::new_from_rigidbody(other, app);

        r1.intersects(&r2)
    }

    /// Simple AABB collision
    pub fn check_collision_aabb(&self, other: &Rigidbody) -> bool {
        //The sides of the rectangles
        let left_a = self.position.x - self.pivot.x;
        let left_b = other.position.x - other.pivot.x;
        let right_a = left_a + (self.bounds.x * self.scale.x);
        let right_b = left_b + (other.bounds.x * other.scale.x);
        let top_a = self.position.y - self.pivot.y;
        let top_b = other.position.y - other.pivot.y;
        let bottom_a = top_a + (self.bounds.y * self.scale.y);
        let bottom_b = top_b + (other.bounds.y * other.scale.y);

        //If any of the sides from A are outside of B
        if bottom_a >= top_b && top_a <= bottom_b && right_a >= left_b && left_a <= right_b {
            return true;
        }

        false
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_vel_getter(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.velocity => 0);
    }

    fn wren_pos_getter(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", self.position => 0);
    }

    fn wren_vel_setter(&mut self, vm: &VM) {
        match vm.get_slot_foreign::<Vec2>(1) {
            Some(vel) => self.velocity = *vel,
            None => {
                LilahTypeError!(Rigidbody, 1, Vec2);
            }
        }
    }

    fn wren_solid_getter(&self, vm: &VM) {
        vm.set_slot_bool(0, self.solid);
    }

    fn wren_solid_setter(&mut self, vm: &VM) {
        match vm.get_slot_bool(1) {
            Some(solid) => self.solid = solid,
            None => {
                LilahTypeError!(Rigidbody, 1, bool);
            }
        }
    }

    fn wren_colliding_getter(&mut self, vm: &VM) {
        if let Some(coll) = self.colliding.clone() {
            vm.set_slot_new_map(0);
            vm.set_slot_string(1, "name");
            vm.set_slot_string(2, coll.name.clone());
            vm.set_map_value(0, 1, 2);
            vm.set_slot_string(1, "uuid");
            vm.set_slot_string(2, coll.uuid.clone());
            vm.set_map_value(0, 1, 2);
        } else {
            vm.set_slot_null(0)
        }
    }

    fn wren_colliding_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => {
                if let Some(coll) = comp.get::<Rigidbody>().colliding.clone() {
                    vm.set_slot_new_map(0);
                    vm.set_slot_string(2, "name");
                    vm.set_slot_string(3, coll.name.clone());
                    vm.set_map_value(0, 2, 3);
                    vm.set_slot_string(2, "uuid");
                    vm.set_slot_string(3, coll.uuid.clone());
                    vm.set_map_value(0, 2, 3);
                } else {
                    vm.set_slot_null(0)
                }
            }
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_vel_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(vel) => comp.get_mut::<Rigidbody>().velocity = *vel,
                None => {
                    LilahTypeError!(Rigidbody, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(pos) => comp.get_mut::<Rigidbody>().position = *pos,
                None => {
                    LilahTypeError!(Rigidbody, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(pos_x) => comp.get_mut::<Rigidbody>().position.x = pos_x,
                None => {
                    LilahTypeError!(Rigidbody, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_pos_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(pos_y) => comp.get_mut::<Rigidbody>().position.y = pos_y,
                None => {
                    LilahTypeError!(Rigidbody, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_vel_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(vel_x) => comp.get_mut::<Rigidbody>().velocity.x = vel_x,
                None => {
                    LilahTypeError!(Rigidbody, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_vel_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(vel_y) => comp.get_mut::<Rigidbody>().velocity.y = vel_y,
                None => {
                    LilahTypeError!(Rigidbody, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_solid_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_bool(2) {
                Some(solid) => comp.get_mut::<Rigidbody>().solid = solid,
                None => {
                    LilahTypeError!(Rigidbody, 2, bool);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_update_vel_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(vel) => comp.get_mut::<Rigidbody>().velocity += *vel,
                None => {
                    LilahTypeError!(Rigidbody, 2, Vec2);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_update_vel_x_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(vel_x) => comp.get_mut::<Rigidbody>().velocity.x += vel_x,
                None => {
                    LilahTypeError!(Rigidbody, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_update_vel_y_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(vel_y) => comp.get_mut::<Rigidbody>().velocity.y += vel_y,
                None => {
                    LilahTypeError!(Rigidbody, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Rigidbody, 1, GameObject);
            }
        }
    }

    fn wren_set_rot_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(rotation) => comp.get_mut::<Rigidbody>().rotation = rotation as f32,
                None => {
                    LilahTypeError!(Rigidbody, 2, f32);
                }
            },
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
            playing: false,
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

    pub fn update(&mut self, dt: f64) {
        if self.playing && self.current_state != String::from("None") {
            if self.current_frame > self.states.get(&self.current_state).unwrap().0 as f64 {
                self.current_frame = 0.0;
            }

            self.current_frame += dt as f64 * self.speed;

            if self.current_frame > self.states.get(&self.current_state).unwrap().0 as f64 {
                self.current_frame = 0.0;
            }
        }
    }

    pub fn update_sprite(&self, sprite: &mut Sprite) {
        if self.current_state != String::from("None") {
            sprite.anim_sprite_sheet(
                self.current_frame as i32,
                self.states.get(&self.current_state).unwrap().1,
            );
        }
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
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
            Some(state) => match self.states.get(&state) {
                Some(s) => {
                    vm.set_slot_new_map(0);
                    vm.set_slot_string(1, state);
                    send_foreign!(vm, "math", "Vec2", Vec2::new(s.0 as f64, s.1 as f64) => 2);
                    vm.set_map_value(0, 1, 2);
                }
                None => {
                    LilahNotFoundError!(Animator, String, state);
                }
            },
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
            Some(state) => match vm.get_slot_foreign::<Vec2>(2) {
                Some(loc) => {
                    self.states.insert(state, (loc.x as i32, loc.y as i32));
                }
                None => {
                    LilahTypeError!(Animator, 2, Vec2);
                }
            },
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
            Some(comp) => match vm.get_slot_string(2) {
                Some(state) => {
                    comp.get_mut::<Animator>().set_state(&state);
                }
                None => {
                    LilahTypeError!(Animator, 2, String);
                }
            },
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_get_state_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign::<GameObject>(1) {
            Some(comp) => match vm.get_slot_string(2) {
                Some(state) => match comp.get::<Animator>().states.get(&state) {
                    Some(s) => {
                        vm.set_slot_new_map(0);
                        vm.set_slot_string(1, state);
                        send_foreign!(vm, "math", "Vec2", Vec2::new(s.0 as f64, s.1 as f64) => 2);
                        vm.set_map_value(0, 1, 2);
                    }
                    None => {
                        LilahNotFoundError!(Animator, String, state);
                    }
                },
                None => {
                    LilahTypeError!(Animator, 2, String);
                }
            },
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_insert_state_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_string(2) {
                Some(state) => match vm.get_slot_foreign::<Vec2>(3) {
                    Some(loc) => {
                        comp.get_mut::<Animator>()
                            .states
                            .insert(state, (loc.x as i32, loc.y as i32));
                    }
                    None => {
                        LilahTypeError!(Animator, 3, Vec2);
                    }
                },
                None => {
                    LilahTypeError!(Animator, 2, String);
                }
            },
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_set_speed_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(speed) => {
                    comp.get_mut::<Animator>().speed = speed;
                }
                None => {
                    LilahTypeError!(Animator, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }

    fn wren_set_frame_from_gameobject(vm: &VM) {
        match vm.get_slot_foreign_mut::<GameObject>(1) {
            Some(comp) => match vm.get_slot_double(2) {
                Some(frame) => {
                    comp.get_mut::<Animator>().current_frame = frame;
                }
                None => {
                    LilahTypeError!(Animator, 2, f64);
                }
            },
            None => {
                LilahTypeError!(Animator, 1, GameObject);
            }
        }
    }
}

impl Text {
    #[rustfmt::skip]
    const DEF_VERTICES: [Vertex; 4] =  [
        Vertex([-0.5, -0.5],  [0.0, 1.0]),
        Vertex([ 0.5, -0.5],  [1.0, 1.0]),
        Vertex([ 0.5,  0.5],  [1.0, 0.0]),
        Vertex([-0.5,  0.5],  [0.0, 0.0]),
    ];

    #[rustfmt::skip]
    const DEF_INDICES: [i32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    pub fn new(t: &str, font: &str) -> Self {
        Self {
            text: t.to_string(),
            font_size: 24,
            font: font.to_string(),
            texture_id: Uuid::new_v4().to_string(),
            changed: true,
            vertex_array: None,
            vertex_buffer: None,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            sort_dirty: true,
            sort: 1000,
        }
    }

    pub fn set_sort(&mut self, s: u32) {
        self.sort_dirty = true;
        self.sort = s;
    }

    pub fn get_sort(&self) -> u32 {
        self.sort
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

    pub fn load(&mut self, app: &mut App, fonts: &HashMap<String, Font>) -> StateUpdateContainer {
        if self.sort_dirty {
            self.sort_dirty = false;
            app.sort_dirty = true;
        }
        
        if self.vertex_array.is_none() {
            unsafe {
                let vao = VertexArray::new();
                vao.bind();

                let vbo = Buffer::new(gl::ARRAY_BUFFER);
                vbo.set_data(&Text::DEF_VERTICES, gl::STATIC_DRAW);

                let ibo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                ibo.set_data(&Text::DEF_INDICES, gl::STATIC_DRAW);

                let pos_attrib = app.text_program.get_attrib_location("position").unwrap();
                set_attribute!(vao, pos_attrib, Vertex::0, gl::FLOAT);
                let color_attrib = app
                    .text_program
                    .get_attrib_location("vertexTexCoord")
                    .unwrap();
                set_attribute!(vao, color_attrib, Vertex::1, gl::FLOAT);

                self.vertex_array = Some(vao);
                self.vertex_buffer = Some(vbo);
            }
        }

        if self.changed {
            self.changed = false;

            if let Some(font) = fonts.get(&self.font) {
                let scale = Scale::uniform(self.font_size as f32);
                let colour = (255, 255, 255);
                let v_metrics = font.v_metrics(scale);

                // layout the glyphs in a line with 20 pixels padding
                let glyphs: Vec<_> = font
                    .layout(&self.text, scale, point(20.0, 20.0 + v_metrics.ascent))
                    .collect();

                // work out the layout size
                let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
                let glyphs_width = {
                    let min_x = glyphs
                        .first()
                        .map(|g| g.pixel_bounding_box().unwrap().min.x)
                        .unwrap();
                    let max_x = glyphs
                        .last()
                        .map(|g| g.pixel_bounding_box().unwrap().max.x)
                        .unwrap();
                    (max_x - min_x) as u32
                };

                // Create a new rgba image with some padding
                let mut image: image::ImageBuffer<Rgba<u8>, Vec<u8>> =
                    DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

                // Loop through the glyphs in the text, positing each one on a line
                for glyph in glyphs {
                    if let Some(bounding_box) = glyph.pixel_bounding_box() {
                        // Draw the glyph into the image per-pixel by using the draw closure
                        glyph.draw(|x, y, v| {
                            image.put_pixel(
                                // Offset the position by the glyph bounding box
                                x + bounding_box.min.x as u32,
                                y + bounding_box.min.y as u32,
                                // Turn the coverage into an alpha value
                                Rgba([255, 255, 255, (v * 255.0) as u8]),
                            )
                        });
                    }
                }

                StateUpdateContainer {
                    textures: Some((self.texture_id.clone(), image)),
                    sfx: None,
                }
            } else {
                let f = self.font.clone();
                LilahNotFoundError!(Text, String, f);
                StateUpdateContainer {
                    textures: None,
                    sfx: None,
                }
            }
        } else {
            StateUpdateContainer {
                textures: None,
                sfx: None,
            }
        }
    }

    pub fn draw(&self, app: &mut App, textures: &HashMap<String, LilahTexture>, t: &Transform) {
        let model = Mat4::IDENTITY
            * Mat4::from_scale_rotation_translation(
                Vec3::new(
                    textures[&self.texture_id].size.x as f32,
                    textures[&self.texture_id].size.y as f32,
                    1.0,
                ) * Vec3::new(t.scale.x as f32, t.scale.y as f32, 1.0),
                Quat::from_rotation_z(t.rotation),
                Vec3::new(
                    t.position.x as f32 + t.pivot.x as f32,// + (textures[&self.texture_id].size.x / 2.0) as f32,
                    t.position.y as f32 + t.pivot.y as f32,// - (textures[&self.texture_id].size.y / 2.0) as f32,
                    0.0,
                ),
            );

        let view = unsafe { *crate::math::VIEW_MATRIX };
        let projection = unsafe { *crate::math::PROJECTION_MATRIX };

        let mvp = projection * view * model;

        unsafe {
            textures[&self.texture_id].bind();

            app.text_program.apply();

            self.vertex_array.as_ref().unwrap().bind();

            let mat_attr =
                gl::GetUniformLocation(app.text_program.id, CString::new("mvp").unwrap().as_ptr());
            gl::UniformMatrix4fv(mat_attr, 1, gl::FALSE as GLboolean, &mvp.to_cols_array()[0]);

            let tint_attr =
                gl::GetUniformLocation(app.text_program.id, CString::new("tint").unwrap().as_ptr());
            gl::Uniform4f(
                tint_attr,
                self.color.r,
                self.color.g,
                self.color.b,
                self.color.a,
            );

            let sort_attr =
                gl::GetUniformLocation(app.text_program.id, CString::new("sort").unwrap().as_ptr());
            gl::Uniform1f(sort_attr, self.sort as f32);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }
    }

    //wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
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
        } else {
            LilahTypeError!(Text, 1, String);
        }
    }

    fn wren_set_font(&mut self, vm: &VM) {
        let a = vm.get_slot_string(1);
        if let Some(a) = a {
            self.set_font(&a);
        } else {
            LilahTypeError!(Text, 1, String);
        }
    }

    fn wren_set_font_size(&mut self, vm: &VM) {
        let a = vm.get_slot_double(1);
        if let Some(a) = a {
            self.set_font_size(a as u32);
        } else {
            LilahTypeError!(Text, 1, f64);
        }
    }

    fn wren_get_text_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            vm.set_slot_string(0, comp.get_mut::<Text>().get_text());
        } else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_get_font_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            vm.set_slot_string(0, comp.get_mut::<Text>().get_font());
        } else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_get_font_size_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            vm.set_slot_double(0, comp.get_mut::<Text>().get_font_size() as f64);
        } else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_set_text_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            let a = vm.get_slot_string(2);
            if let Some(a) = a {
                comp.get_mut::<Text>().set_text(&a);
            } else {
                LilahTypeError!(Text, 2, String);
            }
        } else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_set_font_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            let a = vm.get_slot_string(2);
            if let Some(a) = a {
                comp.get_mut::<Text>().set_font(&a);
            } else {
                LilahTypeError!(Text, 2, String);
            }
        } else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }

    fn wren_set_font_size_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            let a = vm.get_slot_double(2);
            if let Some(a) = a {
                comp.get_mut::<Text>().set_font_size(a as u32);
            } else {
                LilahTypeError!(Text, 2, f64);
            }
        } else {
            LilahTypeError!(Text, 1, GameObject);
        }
    }
}

impl Sprite {
    #[rustfmt::skip]
    const DEF_VERTICES: [Vertex; 4] =  [
        Vertex([-0.5, -0.5],  [0.0, 1.0]),
        Vertex([ 0.5, -0.5],  [1.0, 1.0]),
        Vertex([ 0.5,  0.5],  [1.0, 0.0]),
        Vertex([-0.5,  0.5],  [0.0, 0.0]),
    ];

    #[rustfmt::skip]
    const DEF_INDICES: [i32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    pub fn new(t_id: &str) -> Self {
        Self {
            size: (1, 1),
            base_size: (1, 1),
            index_cut: (0, 0),
            index: (0, 0),
            texture_id: t_id.to_string(),
            vertex_array: None,
            vertex_buffer: None,
            tint: Color::WHITE,
            sort: 0,
            sort_dirty: true
        }
    }

    pub fn set_sort(&mut self, s: u32) {
        self.sort_dirty = true;
        self.sort = s;
    }

    pub fn get_sort(&self) -> u32 {
        self.sort
    }

    pub fn check_dirty(&mut self) -> bool {
        if self.sort_dirty {
            self.sort_dirty = false;
            true
        } else {
            false
        }
    }

    pub fn load(&mut self, app: &mut App, textures: &HashMap<String, LilahTexture>) {
        unsafe {
            let vao = VertexArray::new();
            vao.bind();

            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            vbo.set_data(&Sprite::DEF_VERTICES, gl::DYNAMIC_DRAW);

            let ibo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            ibo.set_data(&Sprite::DEF_INDICES, gl::STATIC_DRAW);

            let pos_attrib = app.default_program.get_attrib_location("position").unwrap();
            set_attribute!(vao, pos_attrib, Vertex::0, gl::FLOAT);
            let color_attrib = app
                .default_program
                .get_attrib_location("vertexTexCoord")
                .unwrap();
            set_attribute!(vao, color_attrib, Vertex::1, gl::FLOAT);

            textures[&self.texture_id].bind();
            app.default_program.set_int_uniform("texture0", 0).unwrap();

            self.vertex_array = Some(vao);
            self.vertex_buffer = Some(vbo);
        }

        if let Some(t) = textures.get(&self.texture_id) {
            self.base_size = (t.size.x as u32, t.size.y as u32);
        } else {
            let id = self.texture_id.clone();
            LilahNotFoundError!(Sprite, Texture, id);
        }

        self.anim_sprite_sheet(self.index_cut.0, self.index_cut.1);
    }

    pub fn get_size(&self) -> (u32, u32) {
        (
            self.base_size.0 / self.size.0,
            self.base_size.1 / self.size.1,
        )
    }

    pub fn cut_sprite_sheet(&mut self, ind: i32, ind2: i32, col: u32, row: u32) {
        self.size = (col, row);
        self.index_cut = (ind, ind2);
        self.index = (0, 0);
    }

    pub fn anim_sprite_sheet(&mut self, ind: i32, ind2: i32) {
        self.index = (
            ind * self.get_size().0 as i32,
            ind2 * self.get_size().1 as i32,
        );
        let ratio = (
            ((self.base_size.0 as f32 / self.size.0 as f32) / self.base_size.0 as f32),
            ((self.base_size.1 as f32 / self.size.1 as f32) / self.base_size.1 as f32),
        );

        fn precision_f32(x: f32, decimals: u32) -> f32 {
            if x == 0. || decimals == 0 {
                0.
            } else {
                let shift = decimals as i32 - x.abs().log10().ceil() as i32;
                let shift_factor = 10_f64.powi(shift) as f32;

                (x * shift_factor).round() / shift_factor
            }
        }

        let zero = (
            precision_f32(
                (ind as f32) / self.size.0 as f32 + (1.0 / self.base_size.0 as f32),
                2,
            ),
            precision_f32(
                (ind2 as f32) / self.size.1 as f32 + (1.0 / self.base_size.0 as f32),
                2,
            ),
        );

        let one = (
            precision_f32(zero.0 + ratio.0 - (1.0 / self.base_size.0 as f32) * 2.0, 2),
            precision_f32(zero.1 + ratio.1 - (1.0 / self.base_size.0 as f32) * 2.0, 2),
        );

        let mut new_verts = Sprite::DEF_VERTICES;
        new_verts[0].1 = [zero.0, one.1];
        new_verts[1].1 = [one.0, one.1];
        new_verts[2].1 = [one.0, zero.1];
        new_verts[3].1 = [zero.0, zero.1];

        unsafe {
            self.vertex_array.as_ref().unwrap().bind();
            self.vertex_buffer
                .as_mut()
                .unwrap()
                .set_data(&new_verts, gl::DYNAMIC_DRAW);
        }
    }

    pub fn draw(&self, app: &mut App, textures: &HashMap<String, LilahTexture>, t: &Transform)  {
        let model = Mat4::IDENTITY
        * Mat4::from_scale_rotation_translation(
            Vec3::new(self.get_size().0 as f32, self.get_size().1 as f32,0.0)
                * Vec3::new(t.scale.x as f32, t.scale.y as f32, 0.0),
            Quat::from_rotation_z(t.rotation),
            Vec3::new(
                t.position.x as f32 + t.pivot.x as f32,
                t.position.y as f32 + t.pivot.y as f32,
                0.0,
            )
        );

        let view = unsafe { *crate::math::VIEW_MATRIX };
        let projection = unsafe { *crate::math::PROJECTION_MATRIX };

        let mvp = projection * view * model;

        unsafe {
            textures[&self.texture_id].activate(gl::TEXTURE0);

            app.default_program.apply();

            self.vertex_array.as_ref().unwrap().bind();

            let mat_attr = gl::GetUniformLocation(
                app.default_program.id,
                CString::new("mvp").unwrap().as_ptr(),
            );
            gl::UniformMatrix4fv(mat_attr, 1, gl::FALSE as GLboolean, &mvp.to_cols_array()[0]);

            let tint_attr = gl::GetUniformLocation(
                app.default_program.id,
                CString::new("tint").unwrap().as_ptr(),
            );
            gl::Uniform4f(
                tint_attr,
                self.tint.r,
                self.tint.g,
                self.tint.b,
                self.tint.a,
            );

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    fn wren_get_size(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", Vec2::new(self.get_size().0 as f64, self.get_size().1 as f64) => 0);
    }

    fn wren_get_index(&self, vm: &VM) {
        send_foreign!(vm, "math", "Vec2", Vec2::new(self.index.0 as f64, self.index.1 as f64) => 0);
    }

    fn wren_get_tint(&self, vm: &VM) {
        vm.set_slot_new_list(0);
        vm.set_slot_double(1, self.tint.r as f64);
        vm.set_list_element(0, 0, 1);
        vm.set_slot_double(1, self.tint.g as f64);
        vm.set_list_element(0, 1, 1);
        vm.set_slot_double(1, self.tint.b as f64);
        vm.set_list_element(0,2, 1);
        vm.set_slot_double(1, self.tint.a as f64);
        vm.set_list_element(0, 3, 1);
    }

    fn wren_set_tint_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            if(comp.has::<Sprite>()) {
                let mut spr = comp.get_mut::<Sprite>();
                vm.get_list_element(2, 0, 3);
                spr.tint.r = vm.get_slot_double(3).unwrap_or(0f64) as f32;
                vm.get_list_element(2, 1, 3);
                spr.tint.g = vm.get_slot_double(3).unwrap_or(0f64) as f32;
                vm.get_list_element(2, 2, 3);
                spr.tint.b = vm.get_slot_double(3).unwrap_or(0f64) as f32;
                vm.get_list_element(2, 3, 3);
                spr.tint.a = vm.get_slot_double(3).unwrap_or(0f64) as f32;
            }
        } else {
            LilahTypeError!(Sprite, 1, GameObject);
        }
    }

    fn wren_cut_sprite_sheet(&mut self, vm: &VM) {
        if let (Some(xy), Some(colrow)) = (
            vm.get_slot_foreign::<Vec2>(1),
            vm.get_slot_foreign::<Vec2>(1),
        ) {
            self.cut_sprite_sheet(xy.x as i32, xy.y as i32, colrow.x as u32, colrow.y as u32);
        } else {
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
            } else {
                LilahTypeError!(Sprite, 2, Vec2);
            }
        } else {
            LilahTypeError!(Sprite, 1, GameObject);
        }
    }

    fn wren_set_sort_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            if let Some(sort) = vm.get_slot_double(2) {
                comp.get_mut::<Sprite>().set_sort(sort as u32);
            } else {
                LilahTypeError!(Sprite, 2, float);
            }
        } else {
            LilahTypeError!(Sprite, 1, GameObject);
        }
    }

    fn wren_cut_sprite_sheet_from_gameobject(vm: &VM) {
        if let Some(comp) = vm.get_slot_foreign_mut::<GameObject>(1) {
            if let (Some(xy), Some(colrow)) = (
                vm.get_slot_foreign::<Vec2>(2),
                vm.get_slot_foreign::<Vec2>(3),
            ) {
                comp.get_mut::<Sprite>().cut_sprite_sheet(
                    xy.x as i32,
                    xy.y as i32,
                    colrow.x as u32,
                    colrow.y as u32,
                );
            } else {
                LilahTypeError!(Sprite, 2, Vec2);
                LilahTypeError!(Sprite, 3, Vec2);
            }
        } else {
            LilahTypeError!(Sprite, 1, GameObject);
        }
    }
}

impl Debug {
    pub fn draw_multi_line(points: Vec<Vec2>, thickness: f64, tint: Color) {
        let line_mesh = crate::math::make_multi_line(&points, thickness);

        let mut vertex_buff = vec!();

        for i in 0..line_mesh.0.len() {
            vertex_buff.push(Vertex([line_mesh.0[i][0].x as f32, line_mesh.0[i][0].y as f32], [0.0, 0.0]));
            vertex_buff.push(Vertex([line_mesh.0[i][1].x as f32, line_mesh.0[i][1].y as f32], [0.0, 0.0]));
            vertex_buff.push(Vertex([line_mesh.0[i][2].x as f32, line_mesh.0[i][2].y as f32], [0.0, 0.0]));
        }

        let model = Mat4::IDENTITY;
        let view = unsafe { *crate::math::VIEW_MATRIX };
        let projection = unsafe { *crate::math::PROJECTION_MATRIX };

        let mvp = projection * view * model;

        unsafe {
            crate::application::DEBUG_PROGRAM.as_mut().expect("program").apply();

            let vao = VertexArray::new();
            vao.bind();

            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            vbo.set_data(vertex_buff.as_slice(), gl::DYNAMIC_DRAW);

            let pos_attrib = crate::application::DEBUG_PROGRAM.as_ref().expect("program").get_attrib_location("position").expect("msg");
            set_attribute!(vao, pos_attrib, Vertex::0, gl::FLOAT);

            let mat_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("mvp").unwrap().as_ptr(),
            );
            gl::UniformMatrix4fv(mat_attr, 1, gl::FALSE as GLboolean, &mvp.to_cols_array()[0]);

            let tint_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("tint").unwrap().as_ptr(),
            );
            gl::Uniform4f(
                tint_attr,
                tint.r,
                tint.g,
                tint.b,
                tint.a,
            );

            let width_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("lineWidth").unwrap().as_ptr(),
            );

            gl::Uniform1d(width_attr, thickness);

            let feather_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("feather").unwrap().as_ptr(),
            );

            gl::Uniform1d(feather_attr, 1.0);

            let segments_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("segments").unwrap().as_ptr(),
            );

            let segments_len_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("segmentCount").unwrap().as_ptr(),
            );

            let flat_segments: Vec<GLfloat> = points.iter()
            .flat_map(|v| vec![v.x as f32, v.y as f32])
            .collect();

            // Send the data to the shader
            gl::Uniform2fv(segments_attr, points.len() as GLsizei, flat_segments.as_ptr());
            gl::Uniform1i(segments_len_attr, points.len() as GLsizei);

            gl::DrawArrays(gl::TRIANGLES, 0, line_mesh.1.len() as i32 * 3);
        } 
    }

    pub fn draw_line(start: Vec2, end: Vec2, tint: Color) {
        let model = Mat4::IDENTITY;
        let view = unsafe { *crate::math::VIEW_MATRIX };
        let projection = unsafe { *crate::math::PROJECTION_MATRIX };

        let mvp = projection * view * model;

        unsafe {
            crate::application::DEBUG_PROGRAM.as_mut().expect("program").apply();

            let vao = VertexArray::new();
            vao.bind();

            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            vbo.set_data(&[Vertex([start.x as f32, start.y as f32],  [0 as f32, 0 as f32]), Vertex([end.x as f32, end.y as f32],  [1 as f32, 0 as f32])], gl::DYNAMIC_DRAW);

            let ibo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            ibo.set_data(&[0, 1], gl::STATIC_DRAW);

            let pos_attrib = crate::application::DEBUG_PROGRAM.as_ref().expect("program").get_attrib_location("position").expect("msg");
            set_attribute!(vao, pos_attrib, Vertex::0, gl::FLOAT);

            let mat_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("mvp").unwrap().as_ptr(),
            );
            gl::UniformMatrix4fv(mat_attr, 1, gl::FALSE as GLboolean, &mvp.to_cols_array()[0]);

            let tint_attr = gl::GetUniformLocation(
                crate::application::DEBUG_PROGRAM.as_ref().expect("program").id,
                CString::new("tint").unwrap().as_ptr(),
            );
            gl::Uniform4f(
                tint_attr,
                tint.r,
                tint.g,
                tint.b,
                tint.a,
            );
            gl::DrawElements(gl::LINES, 2, gl::UNSIGNED_INT, 0 as *const _);
        }
    }

    pub fn wren_draw_line(vm: &VM) {
        if let Some(start) = vm.get_slot_foreign::<Vec2>(1) {
            if let Some(end) = vm.get_slot_foreign::<Vec2>(2) {
                let mut color = Color::new(1.0,1.0,1.0,1.0);
                vm.get_list_element(3, 0, 4);
                color.r = vm.get_slot_double(4).unwrap_or(0f64) as f32;
                vm.get_list_element(3, 1, 4);
                color.g = vm.get_slot_double(4).unwrap_or(0f64) as f32;
                vm.get_list_element(3, 2, 4);
                color.b = vm.get_slot_double(4).unwrap_or(0f64) as f32;
                vm.get_list_element(3, 3, 4);
                color.a = vm.get_slot_double(4).unwrap_or(0f64) as f32;

                //Self::draw_line(*start, *end, color);
                unsafe {
                    crate::application::LINES.push((*start, *end, color));
                }
            } else {
                LilahTypeError!(Debug, 2, Vec2);
            }
        } else {
            LilahTypeError!(Debug, 1, Vec2);
        }
    }
}

impl ComponentBehaviour {
    pub fn new(s: String) -> Self {
        Self {
            component: s.clone(),
            uuid: Uuid::new_v4().to_string()
        }
    }

    pub fn get_component(&self) -> &String {
        &self.component
    }

    //for wren
    fn wren_as_component(&self, vm: &VM) {
        send_foreign!(vm, "game", "Component", Box::new(self.clone()) as Box<dyn Component> => 0);
    }

    pub fn wren_getter_uuid(&mut self, vm: &VM) {
        vm.set_slot_string(0, self.uuid.clone());
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        self.get_size() == other.get_size() && self.index == other.index
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite::new("")
    }
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            bounds: Vec2::ONE,
            pivot: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
            velocity: Vec2::ZERO,
            position: Vec2::ONE,
            colliding: None,
            solid: true,
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

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "Transform", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for Scene {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "Scene", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Component for Sprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "Sprite", self.clone() => slot);
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

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "Rigidbody", self.clone() => slot);
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

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "Animator", self.clone() => slot);
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

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "ComponentBehaviour", self.clone() => slot);
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

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "Text", self.clone() => slot);
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

    fn send_to_wren(&self, slot: usize, vm: &VM) {
        send_foreign!(vm, "game", "Sfx", self.clone() => slot);
    }

    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Tickable<Sprite> for Rigidbody {
    fn tick(&mut self, _: f64, d: &Sprite) {
        let sprite_size = d.get_size();
        
        self.bounds = Vec2::new(sprite_size.0 as f64, sprite_size.1 as f64);
    }
}

impl Tickable<Rigidbody> for Transform {
    fn tick(&mut self, _: f64, d: &Rigidbody) {
        self.position = d.position;
        self.rotation = d.rotation;
    }
}

impl Tickable<Transform> for Rigidbody {
    fn tick(&mut self, _: f64, d: &Transform) {
        self.scale = d.scale;
        self.pivot = d.pivot;
    }
}

impl Tickable<Sprite> for Animator {
    fn tick(&mut self, dt: f64, _: &Sprite) {
        self.update(dt);
    }
}

impl Tickable<Animator> for Sprite {
    fn tick(&mut self, _: f64, d: &Animator) {
        d.update_sprite(self);
    }
}

//Class impls
impl Class for Box<dyn Component> {
    fn initialize(_: &VM) -> Self {
        LilahPanic!(Component, "Cannot instantiate static class");
    }
}

impl Class for Debug {
    fn initialize(_: &VM) -> Self {
        LilahPanic!(Debug, "Cannot instantiate static class");
    }
}

impl Class for Transform {
    fn initialize(vm: &VM) -> Transform {
        if let Some(pos) = vm.get_slot_foreign::<Vec2>(1) {
            Transform::new(*pos)
        } else {
            LilahTypePanic!(ComponentBehaviour, 1, Vec2);
        }
    }
}

impl Class for Scene {
    fn initialize(vm: &VM) -> Self {
        if let Some(t_id) = vm.get_slot_string(1) {
            Scene::new(t_id.to_string())
        } else {
            LilahTypePanic!(Scene, 1, String);
        }
    }
}

impl Class for Sprite {
    fn initialize(vm: &VM) -> Sprite {
        if let Some(t_id) = vm.get_slot_string(1) {
            Sprite::new(t_id.as_str())
        } else {
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
        } else {
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
        } else {
            LilahTypePanic!(Sfx, 1, String);
        }
    }
}

impl Class for Text {
    fn initialize(vm: &VM) -> Text {
        if let (Some(b), Some(c)) = (vm.get_slot_string(1), vm.get_slot_string(2)) {
            Text::new(b.as_str(), c.as_str())
        } else {
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
        instance(getter "pivot") wren_get_pivot,
        instance(setter "position") wren_set_pos,
        instance(setter "scale") wren_set_scale,
        instance(setter "rotation") wren_set_rotation,
        instance(setter "pivot") wren_set_pivot,

        static(fn "set_pivot", 2) wren_set_pivot_from_gameobject,
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
        instance(getter "tint") wren_get_tint,
        instance(getter "texture_id") wren_get_texture_id,
        instance(getter "current_index") wren_get_index,
        instance(fn "cut_sprite_sheet", 2) wren_cut_sprite_sheet,
        static(fn "cut_sprite_sheet", 3) wren_cut_sprite_sheet_from_gameobject,
        static(fn "set_sort", 2) wren_set_sort_from_gameobject,
        static(fn "set_tint", 2) wren_set_tint_from_gameobject
    }

    class("Component") Box<dyn crate::components::Component> => component {
    }

    class("Scene") crate::components::Scene => scene {
        instance(getter "as_component") wren_as_component,
        instance(getter "markers") wren_markers
    }

    class("GameObject") crate::gameobject::GameObject => go {
        instance(fn "getComponent", 1) wren_get_component,
        instance(fn "addComponent", 1) wren_add_component,
        instance(fn "set", 2) wren_set_component,
        instance(getter "id") wren_getter_id,
        instance(getter "components") wren_get_components,
        instance(getter "name") wren_getter_name,
        instance(setter "name") wren_setter_name,
        instance(getter "uuid") wren_getter_uuid
    }

    class("Rigidbody") crate::components::Rigidbody => rigidbody {
        instance(getter "as_component") wren_as_component,
        instance(getter "position") wren_pos_getter,
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
        static(fn "set_solid", 2) wren_set_solid_from_gameobject,
        static(fn "set_rotation", 2) wren_set_rot_from_gameobject
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
        instance(getter "as_component") wren_as_component,
        instance(getter "uuid") wren_getter_uuid
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

    class("Debug") crate::components::Debug => debug {
        static(fn "drawLine", 3) wren_draw_line
    }

    module => game
);

pub fn publish_modules(lib: &mut ModuleLibrary) {
    game::publish_module(lib);
}
