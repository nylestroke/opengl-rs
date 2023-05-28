use gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertVec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl VertVec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            location as gl::types::GLuint, // index of the generic vertex attribute ("layout (location = 0)")
            3,               // the number of components per generic vertex attribute
            gl::FLOAT,       // data type
            gl::FALSE,       // normalized (int-to-float conversion)
            stride as gl::types::GLint, // stride (byte offset between consecutive attributes)
            offset as *const gl::types::GLvoid, // offset of the first component
        );
    }
}

impl From<(f32, f32, f32)> for VertVec3D {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}
