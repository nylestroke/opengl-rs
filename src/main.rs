// Import dependencies
use failure::err_msg;
use render::buffer;
use render::data;
use resources::Resources;
use std::path::Path;

// Import failure crate to handle errors
#[macro_use]
extern crate failure;

// Import render_derive crate to use custom derive macro
#[macro_use]
extern crate render_derive;

// Extern crates are used to import external libraries
extern crate gl; // OpenGL
extern crate sdl2; // SDL2

// Extern crate for vertex attribute pointers
extern crate vec_2_10_10_10;

// Import render module from src/render.rs
pub mod render;
// Import resources module from src/resources.rs
pub mod resources;

// Define a vertex struct with position and color
#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::VertVec3D,
    #[location = "1"]
    color: data::VertRGBA,
}

// Entry point function
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", failure_to_string(e));
        std::process::exit(1);
    }
}

// Function which handles the main loop of the program
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
            color: (1.0, 0.0, 0.0, 1.0).into(),
        }, // bottom right
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            color: (0.0, 1.0, 0.0, 1.0).into(),
        }, // bottom left
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            color: (0.0, 0.0, 1.0, 1.0).into(),
        }, // top
    ];
    // Request OpenGL to give one buffer name (as integer),
    // and write it into vertex buffer object (vbo)
    let vbo = buffer::ArrayBuffer::new(&gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    // Create vertex array object
    let vao = buffer::VertexArray::new(&gl);
    vao.bind();
    vbo.bind();
    Vertex::vertex_attrib_pointers(&gl);
    vbo.unbind();
    vao.unbind();

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
        vao.bind();
        unsafe {
            // Bind the vertex array object
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
