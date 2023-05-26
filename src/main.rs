// Import dependencies
use std::ffi::CString;

// Extern crates are used to import external libraries
extern crate gl; // OpenGL
extern crate sdl2; // SDL2

// Import render module from src/render.rs
pub mod render;

// Entry point function
fn main() {
    // Initialize SDL2
    let sdl = sdl2::init().unwrap();
    // Initialize SDL2 video subsystem
    let video_subsystem = sdl.video().unwrap();

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
        .build()
        .unwrap();

    // Create OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    // Load OpenGL function pointers
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    // Create shaders from vertex and fragment sources
    let vert_shader = render::Shader::from_vert_source(
        &gl,
        &CString::new(include_str!("triangle.vert")).unwrap(),
    )
    .unwrap();
    let frag_shader = render::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("triangle.frag")).unwrap(),
    )
    .unwrap();
    // Linking shaders into program
    let shader_program = render::Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();

    // Create vertex array object with vertecies
    let vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
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
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
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

        // Enable vertex attribute array at index 0
        gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        // Enable vertex attribute array at index 1
        gl.EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            1,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );

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
        for event in sdl.event_pump().unwrap().poll_iter() {
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
}
