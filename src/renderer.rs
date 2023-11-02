use std::{path::Path, ffi::{CString, NulError}, ptr, string::FromUtf8Error};

use gl::types::*;
use image::{ImageError, EncodableLayout, DynamicImage, Rgba};
use thiserror::Error;

use crate::math::Vec2;

#[derive(Clone)]
pub struct LilahTexture {
    pub id: GLuint,
    pub size: Vec2
}

impl Drop for LilahTexture {
    fn drop(&mut self) {
        println!("dropped texture");
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl LilahTexture {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);
        Self { id, size: Vec2::ZERO }
    }

    pub unsafe fn load(&mut self, path: &Path) -> Result<(), ImageError> {
        self.bind();

        let img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(path)?.into_rgba8();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const _,
        );
        self.size = Vec2::new(img.width() as f64, img.height() as f64);
        //gl::GenerateMipmap(gl::TEXTURE_2D);
        Ok(())
    }

    pub unsafe fn load_as_dyn(&mut self, img : image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), ImageError> {
        self.bind();

        //let new_img = DynamicImage::ImageRgba8(img);
        let new_img = img;

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            new_img.width() as i32,
            new_img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            new_img.as_raw().as_ptr() as *const _,
        );
        self.size = Vec2::new(new_img.width() as f64, new_img.height() as f64);
        //gl::GenerateMipmap(gl::TEXTURE_2D);
        Ok(())
    }

    pub unsafe fn load_as_bytes(&mut self, source : &[u8]) -> Result<(), ImageError> {
        self.bind();

        let img = image::load_from_memory(source)?.into_rgba8();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const _,
        );
        self.size = Vec2::new(img.width() as f64, img.height() as f64);
        //gl::GenerateMipmap(gl::TEXTURE_2D);
        Ok(())
    }

    pub unsafe fn set_wrapping(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode as GLint);
    }

    pub unsafe fn set_filtering(&self, mode: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode as GLint);
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id)
    }

    pub unsafe fn activate(&self, unit: GLuint) {
        gl::ActiveTexture(unit);
        self.bind();
    }
}

#[derive(Clone)]
pub struct VertexArray {
    pub id: GLuint,
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }

    pub unsafe fn set_attribute<V: Sized>(
        &self,
        attrib_pos: GLuint,
        components: GLint,
        offset: GLint,
        precision: GLuint
    ) {
        self.bind();
        gl::EnableVertexAttribArray(attrib_pos);
        gl::VertexAttribPointer(
            attrib_pos,
            components,
            precision,
            gl::FALSE,
            std::mem::size_of::<V>() as GLint,
            offset as *const _,
        );
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    fn delete(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

#[macro_export]
macro_rules! set_attribute {
    ($vbo:expr, $pos:tt, $t:ident :: $field:tt, $prec: expr) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
            $prec
        )
    }};
}

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error while compiling shader: {0}")]
    CompilationError(String),
    #[error("Error while linking shaders: {0}")]
    LinkingError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"{0}"}]
    NulError(#[from] NulError),
}

pub struct Shader {
    pub id: GLuint,
}

// impl Drop for Shader {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteShader(self.id);
//         }
//     }
// }

impl Shader {
    pub unsafe fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code).unwrap();
        let shader = Self {
            id: gl::CreateShader(shader_type),
        };
        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);

        // check for shader compilation errors
        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

        if success == 1 {
            Ok(shader)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::CompilationError(log))
        }
    }
}

pub struct ShaderProgram {
    pub id: GLuint,
}

// impl Drop for ShaderProgram {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteProgram(self.id);
//         }
//     }
// }

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, ShaderError> {
        let attrib = CString::new(attrib).unwrap();
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }

    pub unsafe fn set_int_uniform(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        self.apply();
        let uniform = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        Self { id, target }
    }

    pub unsafe fn set_data<D>(&self, data: &[D], usage: GLuint) {
        self.bind();
        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage,
        );
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    fn delete(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

pub type Pos = [f32; 2];
pub type TextureCoords = [f32; 2];

#[repr(C, packed)]
pub struct Vertex(pub Pos, pub TextureCoords);

#[derive(Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }
}

