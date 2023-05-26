// import namespace to avoid repeating `std::ffi` everywhere
use std::ffi::{CStr, CString};

// Newtype wrapper for program
pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

// Implementation of program
impl Program {
    // Function to create program from shaders
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        // Attach shaders to program
        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        // Link program
        unsafe {
            gl.LinkProgram(program_id);
        }

        // Check if linking was successful
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        // Detach shaders from program
        for shader in shaders {
            unsafe {
                gl.DetachShader(program_id, shader.id());
            }
        }

        Ok(Program {
            gl: gl.clone(),
            id: program_id,
        })
    }

    // Function to get program id
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    // Function to set program as used
    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

// Drop trait implementation for program
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

// Newtype wrapper for shader
pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

// Implementation of shader
impl Shader {
    // Function to create shader from source
    pub fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLenum,
    ) -> Result<Shader, String> {
        let id = shader_from_source(&gl, source, kind)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    // Function to create vertex shader from file
    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    // Function to create fragment shader from file
    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    // Function to get shader id
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

// Drop trait implementation for shader
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

// Helper function to compile a shader from string
fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    kind: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    // Obtain shader object id
    let id = unsafe { gl.CreateShader(kind) };

    // Compile shaders from source
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    // Check if compilation was successful
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // If compilation failed, print error message
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            // Get length of error message
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );

            return Err(error.to_string_lossy().into_owned());
        }
    }

    Ok(id)
}

// Function to create whitespace cstring with length
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
