use std::{ops, hash::Hasher, hash::Hash};
use glam::{Mat4, Quat, Vec3};
use ruwren::{Class, VM, create_module, ModuleLibrary};

use crate::{LilahError, LilahPanic, LilahTypeError, components::Rigidbody, application::App};

lazy_mut! {
    pub static mut VIEW_MATRIX: Mat4 = Mat4::IDENTITY;
    pub static mut PROJECTION_MATRIX: Mat4 = Mat4::IDENTITY;
}

pub fn remap (value : f32, from1: f32, to1: f32, from2: f32, to2: f32) -> f32 {
    (value - from1) / (to1 - from1) * (to2 - from2) + from2
}

/// Vector for 2d translations etc.
/// # Examples
/// ## Translations
/// Addition and Subtraction is performed on a Vec2 and Vec2
/// ```rust
/// # use lilah::math::Vec2;
/// let mut a = Vec2::ZERO;
/// a += Vec2::ONE; //{1.0, 1.0}
/// # assert_eq!(a, Vec2::new(1.0, 1.0));
/// let mut b = Vec2::ZERO;
/// b -= Vec2::ONE; //{-1.0, -1.0}
/// # assert_eq!(b, Vec2::new(-1.0, -1.0));
/// ```
/// Multiplication and Division is performed on a Vec2 and Scalar
/// ```rust
/// # use lilah::math::Vec2;
/// let mut a = Vec2::new(2.0, 2.0);
/// a *= 3; //{6.0, 6.0}
/// # assert_eq!(a, Vec2::new(6.0, 6.0));
/// let mut b = Vec2::new(2.0, 2.0);
/// b /= 2; //{1.0, 1.0}
/// # assert_eq!(b, Vec2::new(1.0, 1.0));
/// ```
/// Negate
/// ```rust
/// # use lilah::math::Vec2;
/// let mut a = Vec2::new(5.0, 2.0);
/// a = -a; //{-5.0, -2.0}
/// # assert_eq!(a, Vec2::new(-5.0, -2.0));

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// Allows interop with Wren
impl Class for Vec2 {
    /// Wren constructor
    fn initialize(vm: &VM) -> Self {
        if let (Some(x), Some(y)) = (vm.get_slot_double(1), vm.get_slot_double(2)) {
            Vec2 {
                x: x as f64,
                y: y as f64
            }
        }
        else {
            LilahPanic!(Vec2, "Arg (1) and Arg(2) must be of type Double");
        }
    }
}

/// Hash uses i64 which has side effect of only having unique hashes for Vectors with whole numbers
impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as i64).hash(state);
        (self.y as i64).hash(state);
    }
}

impl Eq for Vec2 { }

impl Vec2 {
    ///x = 0, y = 0
    pub const ZERO: Vec2 = Vec2 {x: 0.0, y: 0.0};
    ///x = 1, y = 1
    pub const ONE: Vec2 = Vec2 {x: 1.0, y: 1.0};
    ///x = 0, y = 1
    pub const UP: Vec2 = Vec2 {x: 0.0, y: 1.0};
    ///x = 1, y = 0
    pub const RIGHT: Vec2 = Vec2 {x: 1.0, y: 0.0};
    ///x = 0, y = -1
    pub const DOWN: Vec2 = Vec2 {x: 0.0, y: -1.0};
    ///x = -1, y = 0
    pub const LEFT: Vec2 = Vec2 {x: -1.0, y: 0.0};
    
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y
        }
    }

    /// Magnitude of Vec2. 
    /// Slower than magnitude_sqr
    pub fn magnitude(&self) -> f64 {
        self.magnitude_sqr().sqrt()
    }
    
    /// Squared Magnitude of Vec2. 
    /// Faster than normal Magnitude and can be used when you don't need an accurate magnitude.
    pub fn magnitude_sqr(&self) -> f64 {
        (self.x*self.x)+(self.y*self.y)
    }
    
    /// Scalar Dot product of two Vectors
    pub fn dot(a: Vec2, b: Vec2) -> f64 {
        a.x*b.x + a.y*b.y
    }
    
    /// Scalar Cross product of two Vectors
    pub fn cross(a: Vec2, b: Vec2) -> f64 {
        a.x*b.y - a.y*b.x
    }
    
    /// Vector normalized to have magnitude between 0 and 1
    pub fn normalized(&self) -> Vec2 {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            Vec2 {x: 0.0, y: 0.0 }
        }
        else {
            Vec2 {x: self.x/magnitude, y: self.y/magnitude }
        }
    }

    /// Applies normalization to Vec
    pub fn normalize(&mut self) {
        let a = self.normalized();
        self.x = a.x;
        self.y = a.y;
    }    
    
    /// interpolation between two Vecs with t between 0, 1
    pub fn lerp(a: Vec2, b: Vec2, t: f64) -> Vec2 {
        Vec2 {x: (a.x + (b.x - a.x) * t), y: (a.y + (b.y - a.y) * t) }
    }

    //for wren
    fn wren_x(&self, vm: &VM) {
        vm.set_slot_double(0, self.x);
    }

    fn wren_y(&self, vm: &VM) {
        vm.set_slot_double(0, self.y);
    }

    fn wren_set_x(&mut self, vm: &VM) {
        if let Some(x) = vm.get_slot_double(1) {
            self.x = x;
        }
        else {
            LilahTypeError!(Vec2, 1, f64);
        }
    }

    fn wren_set_y(&mut self, vm: &VM) {
        if let Some(y) = vm.get_slot_double(1) {
            self.y = y;
        }
        else {
            LilahTypeError!(Vec2, 1, f64);
        }
    }

    fn wren_magnitude(&mut self, vm: &VM) {
        vm.set_slot_double(0, self.magnitude());
    }

    fn wren_magnitude_sqr(&mut self, vm: &VM) {
        vm.set_slot_double(0, self.magnitude_sqr());
    }

    fn wren_normalized(&self, vm: &VM) {
        let a = self.normalized();
        if let Err(e) = vm.set_slot_new_foreign("math", "Vec2", a, 0) {
            LilahError!(Vec2, e);
        }
    }

    fn wren_normalize(&mut self, _vm: &VM) {
        self.normalize();
    }

    fn wren_cross(vm: &VM) {
        let a = vm.get_slot_foreign::<Vec2>(1);
        let b = vm.get_slot_foreign::<Vec2>(2);
        if let (Some(aa), Some(bb)) = (a, b) {
            vm.set_slot_double(0, Vec2::cross(*aa, *bb));
        }
        else {
            LilahTypeError!(Vec2, 1, Vec2);
            LilahTypeError!(Vec2, 2, Vec2);
            vm.set_slot_null(0);
        }
    }

    fn wren_dot(vm: &VM) {
        let a = vm.get_slot_foreign::<Vec2>(1);
        let b = vm.get_slot_foreign::<Vec2>(2);
        if let (Some(aa), Some(bb)) = (a, b) {
            vm.set_slot_double(0, Vec2::dot(*aa, *bb));
        }
        else {
            LilahTypeError!(Vec2, 1, Vec2);
            LilahTypeError!(Vec2, 2, Vec2);
            vm.set_slot_null(0);
        }
    }

    fn wren_lerp(vm: &VM) {
        let a = vm.get_slot_foreign::<Vec2>(1);
        let b = vm.get_slot_foreign::<Vec2>(2);
        let t = vm.get_slot_double(3);
        if let (Some(aa), Some(bb), Some(tt)) = (a, b, t) {
            let _ = vm.set_slot_new_foreign("math", "Vec2", Vec2::lerp(*aa, *bb, tt), 0);
        }
        else {
            LilahTypeError!(Vec2, 1, Vec2);
            LilahTypeError!(Vec2, 2, Vec2);
            LilahTypeError!(Vec2, 3, f64);
            vm.set_slot_null(0);
        }
    }

    fn wren_one(vm: &VM) {
        let _ = vm.set_slot_new_foreign("math", "Vec2", Vec2::ONE, 0);
    }

    fn wren_zero(vm: &VM) {
        let _ = vm.set_slot_new_foreign("math", "Vec2", Vec2::ZERO, 0);
    }

    fn wren_up(vm: &VM) {
        let _ = vm.set_slot_new_foreign("math", "Vec2", Vec2::UP, 0);
    }

    fn wren_down(vm: &VM) {
        let _ = vm.set_slot_new_foreign("math", "Vec2", Vec2::DOWN, 0);
    }

    fn wren_left(vm: &VM) {
        let _ = vm.set_slot_new_foreign("math", "Vec2", Vec2::LEFT, 0);
    }

    fn wren_right(vm: &VM) {
        let _ = vm.set_slot_new_foreign("math", "Vec2", Vec2::RIGHT, 0);
    }

    fn wren_screen_to_world(vm: &VM) {
        if let Some(coord) = vm.get_slot_foreign::<Vec2>(1) {
            let model = 
            Mat4::IDENTITY * 
            Mat4::from_translation( 
                Vec3::new(coord.x as f32, coord.y as f32, 0.0)
            );

            let view = unsafe { *crate::math::VIEW_MATRIX };
            let projection = unsafe { *crate::math::PROJECTION_MATRIX };

            let mvp = (model * view.inverse() * projection);

            let new_point = mvp.to_scale_rotation_translation().2;
            let _ =
            vm.set_slot_new_foreign("math", "Vec2", Vec2::new(new_point.x as f64, new_point.y as f64), 0);
        }
        else {
            LilahTypeError!(Vec2, 1, Vec2);
        }
    }

    fn wren_world_to_screen(vm: &VM) {
        if let Some(coord) = vm.get_slot_foreign::<Vec2>(1) {
            let model = 
            Mat4::IDENTITY * 
            Mat4::from_translation( 
                Vec3::new(coord.x as f32, coord.y as f32, 0.0)
            );

            let view = unsafe { *crate::math::VIEW_MATRIX };
            let projection = unsafe { *crate::math::PROJECTION_MATRIX };

            let mvp = (projection * view.inverse() * model);

            let new_point = mvp.to_scale_rotation_translation().2;
            let point_to_screen = Vec2::new(
                new_point.x as f64,
                new_point.y as f64
                //remap(new_point.x, -1.0, 0.0, 1.0, 800.0) as f64/2.0, 
                //remap(new_point.y, -1.0, 0.0, 1.0, 600.0) as f64/2.0
            );
            let _ =
            vm.set_slot_new_foreign("math", "Vec2", point_to_screen, 0);
        }
        else {
            LilahTypeError!(Vec2, 1, Vec2);
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x+other.x, y: self.y+other.y }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self { x: self.x+other.x, y: self.y+other.y }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x-other.x, y: self.y-other.y }
    }
}

impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self { x: self.x-other.x, y: self.y-other.y }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f64) -> Vec2 {
        Vec2 { x: self.x*other, y: self.y*other }
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self { x: self.x*other, y: self.y*other }
    }
}

impl ops::MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, other: i32) {
        *self = Self { x: self.x*other as f64, y: self.y*other as f64 }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f64) -> Vec2 {
        Vec2 { x: self.x/other, y: self.y/other }
    }
}

impl ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, other: f64) {
        *self = Self { x: self.x/other, y: self.y/other }
    }
}

impl ops::DivAssign<i32> for Vec2 {
    fn div_assign(&mut self, other: i32) {
        *self = Self { x: self.x/other as f64, y: self.y/other as f64 }
    }
}

impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: i32) -> Vec2 {
        Vec2 { x: self.x*other as f64, y: self.y*other as f64 }
    }
}

impl ops::Div<i32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: i32) -> Vec2 {
        Vec2 { x: self.x/other as f64, y: self.y/other as f64 }
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        self*-1
    }
}

///Displays in format (x=val, y=val)
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

pub struct Rect {
    pub points : Vec<Vec2>
}

impl Rect {
    pub fn new_from_rigidbody(body: &Rigidbody, app: &App) -> Self {
        let model = 
        Mat4::IDENTITY * 
        Mat4::from_rotation_translation(
            Quat::from_rotation_z(0.0),
            Vec3::new(body.position.x as f32, body.position.y as f32, 0.0)
        );

        let view = unsafe { *crate::math::VIEW_MATRIX };
        let projection = unsafe { *crate::math::PROJECTION_MATRIX };

        let mvp = projection * view * model;

        let a = mvp * glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let b = mvp * glam::Vec4::new(body.bounds.x as f32, 0.0, 0.0, 1.0);
        let c = mvp * glam::Vec4::new(body.bounds.x as f32, -body.bounds.y as f32, 0.0, 1.0);
        let d = mvp * glam::Vec4::new(0.0, -body.bounds.y as f32, 0.0, 1.0);

        Self {
            points: vec![
                Vec2::new(a.x as f64, a.y as f64),
                Vec2::new(b.x as f64, b.y as f64),
                Vec2::new(c.x as f64, c.y as f64),
                Vec2::new(d.x as f64, d.y as f64),
            ]
        }
    }

    fn get_edges(r: &Rect) -> Vec<Vec2> {
        let mut edges = vec!();

        for i in 0..r.points.len() {
            edges.push(r.points[(i+1).rem_euclid(r.points.len())]-r.points[i]);
        }

        edges
    }

    fn get_projection(r: &Rect, axis: &Vec2) -> (f64, f64) {
        let mut projections = vec!();

        for point in &r.points {
            projections.push(Vec2::dot(*point, *axis));
        }

        (*projections.iter().min_by(|a, b| a.total_cmp(b)).unwrap(), *projections.iter().max_by(|a, b| a.total_cmp(b)).unwrap())
    }

    fn process_edges(edges : &mut Vec<Vec2>) {
        for e in edges {
            let new_e = e.clone();
            *e = Vec2::new(new_e.x, new_e.y).normalized();
        }
    }

    pub fn intersects(&self, other: &Rect) -> (bool, Vec2) {
        let mut edges = Rect::get_edges(self);
        edges.append(&mut Rect::get_edges(&other));
        Rect::process_edges(&mut edges);
        let mut intersecting_axis = vec!();

        for e in &edges {
            let proj_a = Rect::get_projection(self, e);
            let proj_b = Rect::get_projection(other, e);

            if !(proj_a.0.min(proj_a.1) <= proj_b.0.max(proj_b.1) &&
                proj_b.0.min(proj_b.1) <= proj_a.0.max(proj_a.1)) {
                return (false, Vec2::ZERO);
            }
            
            if (proj_a.0.max(proj_a.1) - proj_b.0.min(proj_b.1)) != 0.0 {
                intersecting_axis.push((e, (proj_a.0.max(proj_a.1) - proj_b.0.min(proj_b.1))));
            }
        }  

        intersecting_axis.sort_by(|a, b| a.1.total_cmp(&b.1));

        (true, *intersecting_axis[0].0*intersecting_axis[0].1)
    }
}

create_module! (
    class("Vec2") crate::math::Vec2 => vec2 {
        instance(getter "x") wren_x,
        instance(getter "y") wren_y,
        instance(setter "x") wren_set_x,
        instance(setter "y") wren_set_y,

        static(getter "one") wren_one,
        static(getter "zero") wren_zero,
        static(getter "up") wren_up,
        static(getter "down") wren_down,
        static(getter "left") wren_left,
        static(getter "right") wren_right,

        instance(fn "magnitude", 0) wren_magnitude,
        instance(fn "magnitude_sqr", 0) wren_magnitude_sqr,
        instance(fn "normalized", 0) wren_normalized,
        instance(fn "normalize", 0) wren_normalize,

        static(fn "cross", 2) wren_cross,
        static(fn "dot", 2) wren_dot,
        static(fn "lerp", 3) wren_lerp,
        static(fn "screen_to_world_space", 1) wren_screen_to_world,
        static(fn "world_to_screen_space", 1) wren_world_to_screen
    }

    module => math
);

pub fn publish_modules(lib : &mut ModuleLibrary) {
    math::publish_module(lib);
}
