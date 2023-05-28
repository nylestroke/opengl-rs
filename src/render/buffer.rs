// Import dependencies
use gl;

// Trait to represent the buffer type
pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

// Struct that represents a array buffer
pub struct Buffer<B>
where
    B: BufferType,
{
    // The gl context
    gl: gl::Gl,
    // The id of the vertex buffer object
    vbo: gl::types::GLuint,
    // The PhantomData marker
    _marker: ::std::marker::PhantomData<B>,
}

// Implement the array buffer struct
impl<B> Buffer<B>
where
    B: BufferType,
{
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            // Generate a new vertex buffer object
            gl.GenBuffers(1, &mut vbo);
        }

        Self {
            gl: gl.clone(),
            vbo,
            _marker: std::marker::PhantomData,
        }
    }

    // Function to bind the array buffer
    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    // Function to unbind the array buffer
    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    // Function to draw the data
    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,                                                   // target
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                           // usage
            );
        }
    }
}

// Implement drop trait for the array buffer struct
impl<B> Drop for Buffer<B>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

// Struct that represents a vertex array
pub struct VertexArray {
    gl: gl::Gl,
    vao: gl::types::GLuint,
}

// Implement the vertex array struct
impl VertexArray {
    // Constructor for the vertex array struct
    pub fn new(gl: &gl::Gl) -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            gl: gl.clone(),
            vao,
        }
    }

    // Function to bind the vertex array
    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    //
    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

// Implement drop trait for the vertex array struct
impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &mut self.vao);
        }
    }
}

// Struct that represents a buffer type array
pub struct BufferTypeArray;

// Implement the buffer type trait for the buffer type array struct
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

// Struct that represents a buffer type element array
pub struct BufferTypeElementArray;

// Implement the buffer type trait for the buffer type element array struct
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

// Public type aliases
pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;
