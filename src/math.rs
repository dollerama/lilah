use std::{collections::HashMap, hash::{Hash, Hasher}, mem, ops};
use glam::{Mat4, Quat, Vec3, Vec4};
use ruwren::{Class, VM, create_module, ModuleLibrary};
use std::f64::consts::PI;
use crate::{application::App, components::Rigidbody, renderer::{Shader, ShaderProgram}, LilahError, LilahPanic, LilahTypeError};

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

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let a = integer_decode(self.x);
        let b = integer_decode(self.y);
        a.0.hash(state);
        a.1.hash(state);
        a.2.hash(state);
        b.0.hash(state);
        b.1.hash(state);
        b.2.hash(state);
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

    /// Perpendicular Vector
    pub fn perp(a: Vec2) -> Vec2 {
        Vec2::new(a.y, -a.x)
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

    pub fn get_intersection(a: (Vec2, Vec2), b: (Vec2, Vec2)) -> Option<Vec2> {
        let xdiff = Vec2::new(a.0.x-a.1.x, b.0.x-b.1.x);
        let ydiff = Vec2::new(a.0.y-a.1.y, b.0.y-b.1.y); 
        
        let div = Vec2::cross(xdiff, ydiff);
        
        if div == 0.0 {
            None
        } else {
            let d = Vec2::new(Vec2::cross(a.0, a.1), Vec2::cross(b.0, b.1));
            let x = Vec2::cross(d, xdiff) / div;
            let y = Vec2::cross(d, ydiff) / div;
            
            Some(Vec2::new(x, y))
        }
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
            if a.is_none() { LilahTypeError!(Vec2, 1, Vec2); }
            if b.is_none() { LilahTypeError!(Vec2, 2, Vec2); }
            if t.is_none() { LilahTypeError!(Vec2, 3, f64); }
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

pub fn make_line(start: Vec2, end: Vec2, thickness: f64, output: bool) -> [Vec2; 4] {
    let offset = Vec2::perp(end-start).normalized() * thickness;
    let a = Vec2::new(start.x, start.y) + offset;
    let b = Vec2::new(end.x, end.y) + offset;
    let c = Vec2::new(end.x, end.y) - offset;
    let d = Vec2::new(start.x, start.y) - offset;
    
    if output {
        println!("[{}, {}, {}, {}, {}]", a, b, c, d, a);
    }
    [a, b, c, d]
}

pub fn make_multi_line(lines: &Vec<Vec2>, thickness: f64) -> (Vec<[Vec2; 3]>, Vec<i32>) {
    let mut point = 0;
    
    let mut lines_tmp = vec!();
    let mut curves_tmp = vec!();
    
    loop {
        if point > lines.len()-1 {
            break;
        }
        if point == 0 {
            point += 1;
            continue;
        }
        
        if point == 1 {
            point += 1;
            continue;
        }
        
        let pp = (lines[point-2], lines[point-1], lines[point]);
        
        let a = make_line(pp.0, pp.1, thickness, false);
        let b = make_line(pp.1, pp.2, thickness, false);
        
        let angle = Vec2::dot(Vec2::perp(pp.1-pp.0).normalized(), (pp.2-pp.1).normalized());

        let inter = Vec2::get_intersection((a[3], a[2]), (b[3], b[2])).unwrap();
        let correct = (inter-a[2]).magnitude();
        let lines_gen = if angle < 0.0 {
            ( 
                make_line(pp.0, pp.1 - (pp.1-pp.0).normalized() * correct, thickness, false),
                make_line(pp.1 + (pp.2 - pp.1).normalized() * correct, pp.2, thickness, false)
            )
        } else {
            (
                make_line(pp.0, pp.1 - (pp.1-pp.0).normalized() * correct, thickness, false),
                make_line(pp.1 + (pp.2 - pp.1).normalized() * correct, pp.2, thickness, false)
            )
        };
        
        //start cap
        if point == 2 {
            let center = pp.0;
            let mut prev = lines_gen.0[3];
            let correct = lines_gen.0[3]-pp.0;
            let mut angle = (correct.y).atan2(correct.x);
            
            for i in 0..=5 {
                let next = Vec2::new(center.x+angle.cos() as f64*correct.magnitude(), center.y+angle.sin() as f64*correct.magnitude()); 
                curves_tmp.push(([prev, next, center], 0));
                angle += std::f64::consts::PI/5.0;
                prev = next;
            }
        }
        //end cap
        if point == lines.len()-1 {
            let center = pp.2;
            let mut prev = lines_gen.1[2];
            let correct = lines_gen.1[2]-pp.2;
            let mut angle = (correct.y).atan2(correct.x);
            
            for i in 0..=5 {
                let next = Vec2::new(center.x+angle.cos() as f64*correct.magnitude(), center.y+angle.sin() as f64*correct.magnitude()); 
                curves_tmp.push(([prev, next, center], point));
                angle -= std::f64::consts::PI/5.0;
                prev = next;
            }
        }

        lines_tmp.push(lines_gen.0);
        lines_tmp.push(lines_gen.1);
    
        let angle2 = Vec2::dot((lines_gen.0[1]-lines_gen.0[2]).normalized(), (lines_gen.1[0]-lines_gen.0[2]).normalized())*2.0;
        
        if angle2 < 0.0 {
            let angle2 = Vec2::dot((lines_gen.0[2]-lines_gen.0[1]).normalized(), (lines_gen.1[0]-lines_gen.0[1]).normalized())*2.0;
            let test = lines_gen.0[1] - lines_gen.0[2];
            let mut start_angle = (test.y).atan2(test.x)+angle2/4.0 as f64;//+angle2;
            let correct2 = (lines_gen.0[2]-lines_gen.0[1]); 
            let mut prev = lines_gen.0[1];

            for i in 0..=3 {
                let mid_curve = Vec2::new(lines_gen.0[2].x+(start_angle).cos() as f64*correct2.magnitude(), lines_gen.0[2].y+(start_angle).sin() as f64*correct2.magnitude());
                curves_tmp.push(([prev, mid_curve, lines_gen.0[2]], point-1));
                prev = mid_curve;
                
                start_angle += angle2/4.0 as f64;
            }
            curves_tmp.push(([lines_gen.0[2], lines_gen.1[0], prev], point-1));
        } else {
            let angle2 = Vec2::dot((lines_gen.0[2]-lines_gen.0[1]).normalized(), (lines_gen.1[3]-lines_gen.0[2]).normalized())*2.0;
            let test = lines_gen.0[2] - lines_gen.0[1];
            let mut start_angle = (test.y).atan2(test.x)+(angle2);
            let correct2 = (lines_gen.1[3]-lines_gen.0[1]); 
            let mut prev = lines_gen.1[3];
            
            for i in 0..=3 {
                let mid_curve = Vec2::new(lines_gen.0[1].x+(start_angle).cos() as f64*correct2.magnitude(), lines_gen.0[1].y+(start_angle).sin() as f64*correct2.magnitude());
                curves_tmp.push(([prev, mid_curve, lines_gen.1[0]], point-1));
                prev = mid_curve;
                
                start_angle -= angle2/4.0 as f64;
            }
            curves_tmp.push(([lines_gen.0[1], prev, lines_gen.0[2]], point-1));
        }
        
        point += 1;
    }
    
    let mut result_lines = vec![];
    
    for i in curves_tmp.iter() {
        if i.1 == 0 {
            result_lines.push(i.0);
        } else {
            break;
        }
    }
    
    result_lines.push([lines_tmp[0][2], lines_tmp[0][0], lines_tmp[0][1]]);
    result_lines.push([lines_tmp[0][3], lines_tmp[0][0], lines_tmp[0][2]]);

    let mut i = 2;
    let mut j = 2;
    loop {
        if i >= lines_tmp.len()-1 {
            result_lines.push(
                [lines_tmp[lines_tmp.len()-1][2], lines_tmp[lines_tmp.len()-1][0], lines_tmp[lines_tmp.len()-1][1]]
            );
            result_lines.push(
                [lines_tmp[lines_tmp.len()-1][3], lines_tmp[lines_tmp.len()-1][0], lines_tmp[lines_tmp.len()-1][2]]
            );
            
            for i in curves_tmp.iter() {
                if i.1 == lines.len()-1 {
                    result_lines.push(i.0);
                }
            }
            break;
        }
        for k in curves_tmp.iter() {
            if k.1 == j-1 {
                result_lines.push(k.0);
            }
        }

        result_lines.push([lines_tmp[i-1][2], lines_tmp[i-1][0], lines_tmp[i][1]]);
        result_lines.push([lines_tmp[i][3], lines_tmp[i][0], lines_tmp[i-1][2]]);
        
        
        for k in curves_tmp.iter() {
            if k.1 == j {
                result_lines.push(k.0);
            }
        }
        i += 2;
        j += 1;
    }
    
    let mut indices = vec!();
    let mut check = HashMap::new();
    for k in result_lines.iter() {
        for vert in k {
            if let Some(ind) = check.get(vert) {
                indices.push(*ind);  
            } else {
                if check.len() == 0 {
                    let num = 0;
                    indices.push(num);  
                    check.insert(vert, num);
                } else {
                    let num = check.len() as i32;
                    check.insert(vert, num);
                    indices.push(num);  
                }
            }
        }
    }

    (result_lines, indices)
}

pub struct Rect {
    pub points : Vec<Vec2>
}

impl Rect {
    pub fn new_from_rigidbody(body: &Rigidbody, _app: &App) -> Self {
        let model = 
        Mat4::IDENTITY * 
        Mat4::from_scale_rotation_translation( 
            Vec3::new(body.bounds.x as f32 * body.scale.x as f32, body.bounds.y  as f32 * body.scale.y as f32, 1.0),
            Quat::from_rotation_z(body.rotation),
            Vec3::new(body.position.x as f32 + body.pivot.x as f32, body.position.y as f32 + body.pivot.y as f32, 0.0)
        );

        let view = unsafe { *crate::math::VIEW_MATRIX };
        let projection = unsafe { *crate::math::PROJECTION_MATRIX };

        let mvp =  view * model;

        let a = mvp * glam::Vec4::new(-0.5f32, 0.5f32, 0.0, 1.0);
        let b = mvp * glam::Vec4::new(0.5f32, 0.5f32, 0.0, 1.0);
        let c = mvp * glam::Vec4::new(0.5f32, -0.5f32, 0.0, 1.0);
        let d = mvp * glam::Vec4::new(-0.5f32, -0.5f32, 0.0, 1.0);

        Self {
            points: vec![
                Vec2::new(a.x as f64, a.y as f64),
                Vec2::new(b.x as f64, b.y as f64),
                Vec2::new(c.x as f64, c.y as f64),
                Vec2::new(d.x as f64, d.y as f64),
            ]
        }
    }

   // Assuming Vec2 is a struct with fields x and y, and methods like dot, normalized, and basic arithmetic operations implemented.

    fn get_edges(r: &Rect) -> Vec<Vec2> {
        let mut edges = vec![];

        for i in 0..r.points.len() {
            let next_index = (i + 1) % r.points.len();
            edges.push(r.points[next_index] - r.points[i]);
        }

        edges
    }

    fn get_projection(r: &Rect, axis: &Vec2) -> (f64, f64) {
        let mut min_proj = f64::MAX;
        let mut max_proj = f64::MIN;

        for point in &r.points {
            let proj = Vec2::dot(*point, *axis);
            if proj < min_proj {
                min_proj = proj;
            }
            if proj > max_proj {
                max_proj = proj;
            }
        }

        (min_proj, max_proj)
    }

    fn process_edges(edges: &mut Vec<Vec2>) {
        for e in edges.iter_mut() {
            *e = e.normalized();
        }
    }

    pub fn intersects(&self, other: &Rect) -> (bool, Vec2) {
        //println!("r -> {:?}", self.points);

        let mut edges = Rect::get_edges(self);
        edges.append(&mut Rect::get_edges(other));
        Rect::process_edges(&mut edges);
        //let mut intersecting_axis = vec![];
        let mut mtv_distance = f64::MAX;
        let mut mtv_axis = Vec2::ZERO;

        for e in &edges {
            let proj_a = Rect::get_projection(self, e);
            let proj_b = Rect::get_projection(other, e);

            if !(proj_a.0 <= proj_b.1 && proj_b.0 <= proj_a.1) {
                return (false, Vec2::ZERO);
            }

            let overlap = if proj_a.1 > proj_b.1 {
                proj_b.1 - proj_a.0
            } else {
                proj_a.1 - proj_b.0
            };

            if overlap.abs() < mtv_distance {
                mtv_distance = overlap.abs();
                mtv_axis = *e;
            }
        }

        let mtv_axis2 = unsafe { *crate::math::PROJECTION_MATRIX } * Vec4::new(mtv_axis.x as f32, mtv_axis.y as f32, 0.0, 0.0); 

        let mtv = Vec2::new(mtv_axis2.x as f64, mtv_axis2.y as f64) * mtv_distance;
        
        (true, mtv)
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
