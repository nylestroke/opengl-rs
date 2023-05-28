// Import dependencies
use failure::err_msg;
use render::data;
use resources::Resources;
use std::path::Path;

// Import failure crate to handle errors
#[macro_use]
extern crate failure;

// Extern crates are used to import external libraries
extern crate gl; // OpenGL
extern crate sdl2; // SDL2

// Import render module from src/render.rs
pub mod render;
// Import resources module from src/resources.rs
pub mod resources;

// Define a vertex struct with position and color
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::VertVec3D,
    color: data::VertVec3D,
}

// Implement vertex attribute pointers for Vertex struct
impl Vertex {
    // Function that takes a reference to gl::Gl struct and enables vertex attribute array
    fn vertex_attrib_pointers(gl: &gl::Gl) {
        let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

        let location = 0; // "layout (location = 0)" in vertex shader
        let offset = 0; // offset of the first component

        unsafe {
            data::VertVec3D::vertex_attrib_pointer(gl, stride, location, offset);
        }

        let location = 1; // "layout (location = 1)" in vertex shader
        let offset = offset + std::mem::size_of::<data::VertVec3D>(); // offset of the first component

        unsafe {
            data::VertVec3D::vertex_attrib_pointer(gl, stride, location, offset);
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", failure_to_string(e));
        std::process::exit(1);
    }
}

// Entry point function
fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets")).map_err(err_msg)?;

    // Initialize SDL2
    let sdl = sdl2::init().map_err(err_msg)?;
    // Initialize SDL2 video subsystem
    let video_subsystem = sdl.video().map_err(err_msg)?;

    //  Set OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    // Set OpenGL version to 4.5
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    // Create a window
    let window = video_subsystem
        .window("OpenGL Window - Rust", 800, 700)
        .opengl() // Add OpenGL flag
        .resizable()
        .position_centered()
        .build()?;
    // Create OpenGL context
    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    // Load OpenGL function pointers
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    // Create shaders from vertex and fragment sources
    // Linking shaders into program
    let shader_program =
        render::Program::from_res(&gl, &res, "shaders/triangle").map_err(err_msg)?;

    // Create vertex array object with vertecies
    let vertices: Vec<Vertex> = vec![
        // positions      // colors
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            color: (1.0, 0.0, 0.0).into(),
        }, // bottom right
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            color: (0.0, 1.0, 0.0).into(),
        }, // bottom left
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            color: (0.0, 0.0, 1.0).into(),
        }, // top
    ];
    // Request OpenGL to give one buffer name (as integer),
    // and write it into vertex buffer object (vbo)
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    unsafe {
        // Bind the buffer to the array buffer
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,                                                          // target
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    // Create vertex array object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    unsafe {
        // Bind the vertex array object
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Enable vertex attribute array
        Vertex::vertex_attrib_pointers(&gl);

        // Unbind the buffer and vertex array object
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    // Set shared state for window
    unsafe {
        gl.Viewport(0, 0, window.size().0 as i32, window.size().1 as i32); // set viewport
        gl.ClearColor(0.24, 0.7, 0.5, 1.0);
    }

    'main: loop {
        // Handle events
        for event in sdl.event_pump().map_err(err_msg)?.poll_iter() {
            match event {
                // Quit event or escape key pressed
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                // Update window viewport after resize event
                sdl2::event::Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::Resized(width, height) => unsafe {
                        gl.Viewport(0, 0, width, height);
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        // Clear the screen to the background color
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Set the shader program as used
        shader_program.set_used();

        // Draw triangle
        unsafe {
            // Bind the vertex array object
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        // Swap the window
        window.gl_swap_window();
    }

    Ok(())
}

// Function that takes any object that implements failure::Fail and prints out the chain of all causes:
pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            let _ = writeln!(&mut result, "   Which caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}
