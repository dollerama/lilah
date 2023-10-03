use std::{ops, hash::Hasher, hash::Hash};

use ruwren::{Class, get_slot_checked, VM, create_module, ModuleLibrary, send_foreign};

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
        let x = get_slot_checked!(vm => num 1);
        let y = get_slot_checked!(vm => num 2);
        Vec2 {
            x: x as f64,
            y: y as f64
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
        self.x = get_slot_checked!(vm => num 1);
    }

    fn wren_set_y(&mut self, vm: &VM) {
        self.y = get_slot_checked!(vm => num 1);
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
            eprintln!("{}", e);
        }
    }

    fn wren_normalize(&mut self, vm: &VM) {
        self.normalize();
    }

    fn wren_cross(vm: &VM) {
        let a = vm.get_slot_foreign::<Vec2>(1);
        let b = vm.get_slot_foreign::<Vec2>(2);
        if let (Some(aa), Some(bb)) = (a, b) {
            vm.set_slot_double(0, Vec2::cross(*aa, *bb));
        }
        else {
            eprintln!("Vec2 must take (Vec2, Vec2)");
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
            eprintln!("Vec2 must take (Vec2, Vec2)");
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
            eprintln!("Vec2 must take (Vec2, Vec2)");
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
        static(fn "lerp", 3) wren_lerp
    }
    
    module => math
);

pub fn publish_modules(lib : &mut ModuleLibrary) {
    math::publish_module(lib);
}
