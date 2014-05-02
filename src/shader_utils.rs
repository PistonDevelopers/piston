//! Helper functions for dealing with shaders.

// External crates.
use opengles::gl2;
use std::io;
use std::path;

/// Compiles a vertex shader from file and fails if not succeeding.
pub fn compile_shader_file(
    shader_type: gl2::GLenum, 
    file: &str
) -> gl2::GLuint {
    with_shader_file(file, |res| {
        match res {
            Err(err) => fail!("Failed opening shader file '{}':\r\n{}", file, err),
            Ok(source) => 
                match compile_shader(shader_type, source) {
                    Err(err) => fail!("Failed compiling shader file '{}':\r\n{}", file, err),
                    Ok(shader) => shader
                },
        }
    })
}

/// Splits lines into byte slices.
pub fn with_shader_source<T>(source: &str, f: |&[&[u8]]| -> T) -> T {
    let lines: Vec<&[u8]> = source.split('\n')
                .map(|line| line.as_bytes()).collect();
    f(lines.as_slice())
}

/// Reads a shader file and creates data required to compile.
pub fn with_shader_file<T>(file: &str, f: |io::IoResult<&[&[u8]]>| -> T) -> T {
    let path = path::Path::new(file);
    let file = io::File::open(&path);
    match file {
        Err(err) => f(Err(err)),
        Ok(file) => {
            // Read the lines and tell errors if they occur.
            let mut reader = io::BufferedReader::new(file);
            let mut lines: Vec<~str> = Vec::new();
            for line in reader.lines() {
                if line.is_err() {
                    return f(Err(line.unwrap_err()));
                }

                lines.push(line.unwrap());
            }

            let lines: Vec<&[u8]> = lines.iter()
                .map(|line| line.as_bytes()).collect();
            f(Ok(lines.as_slice()))
        }
    }
}

/// Compiles a shader.
/// Returns a shader or a message with the error.
pub fn compile_shader(
    shader_type: gl2::GLenum,
    source: &[&[u8]]
) -> Result<gl2::GLuint, ~str> {
    let shader = gl2::create_shader(shader_type);
    gl2::shader_source(shader, source);
    gl2::compile_shader(shader);
    if gl2::get_shader_iv(shader, gl2::COMPILE_STATUS) 
    == gl2::TRUE as gl2::GLint {
        Ok(shader)
    } else {
        let msg = gl2::get_shader_info_log(shader);
        gl2::delete_shader(shader);
        Err(msg)
    }
}


