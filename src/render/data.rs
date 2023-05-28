#[allow(non_camel_case_types)]
// Import dependencies
use gl;

// Struct that represents a vertex with a position and a color
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertVec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Implement a constructor for the vertex struct
impl VertVec3D {
    // Function which creates a new vertex
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // Function which enables and sets the vertex attribute pointers
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        // Enable the vertex attribute array at the given location
        gl.EnableVertexAttribArray(location as gl::types::GLuint); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            location as gl::types::GLuint, // index of the generic vertex attribute ("layout (location = 0)")
            3,                             // the number of components per generic vertex attribute
            gl::FLOAT,                     // data type
            gl::FALSE,                     // normalized (int-to-float conversion)
            stride as gl::types::GLint,    // stride (byte offset between consecutive attributes)
            offset as *const gl::types::GLvoid, // offset of the first component
        );
    }
}

// Implement a constructor for the vertex struct
impl From<(f32, f32, f32)> for VertVec3D {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

// Struct that represents a 4 demensional vector with 2 bits for first and 10 bits for the rest
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertRGBA {
    pub inner: ::vec_2_10_10_10::Vector,
}

// Implement a constructor for the vertex struct
impl From<(f32, f32, f32, f32)> for VertRGBA {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Self {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
        }
    }
}

// Implement the vertex-rgba struct
impl VertRGBA {
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        // Enable the vertex attribute array at the given location
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4, // the number of components per generic vertex attribute
            gl::UNSIGNED_INT_2_10_10_10_REV, // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// Struct that represents i8 vertex
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertI8 {
    pub x: i8,
}

// Implement the vertex-i8 struct
impl VertI8 {
    // Function which creates a new vertex
    pub fn new(x: i8) -> Self {
        Self { x }
    }

    // Function which enables and sets the vertex attribute pointers
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(
            location as gl::types::GLuint,
            1,        // the number of components per generic vertex attribute
            gl::BYTE, // data type
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// Implement a constructor for the vertex struct
impl From<i8> for VertI8 {
    fn from(other: i8) -> Self {
        VertI8::new(other)
    }
}

// Struct that represents i8 vertex with a float
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertI8Float {
    pub x: i8,
}

impl VertI8Float {
    // Function which creates a new vertex
    pub fn new(x: i8) -> Self {
        Self { x }
    }

    // Function which enables and sets the vertex attribute pointers
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        // Enable the vertex attribute array at the given location
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            1,        // the number of components per generic vertex attribute
            gl::BYTE, // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

// Implement a constructor for the vertex struct
impl From<i8> for VertI8Float {
    /// Create this data type from i8
    fn from(other: i8) -> Self {
        VertI8Float::new(other)
    }
}
