//! Helper functions for dealing with shaders.

// External crates.
use gl;
use gl::types::{
    GLchar,
    GLenum,
    GLint,
    GLuint,
};

//use std::io;
//use std::path;
use std::ptr;

/// Compiles a vertex shader from file and fails if not succeeding.
// pub fn compile_shader_file(
//     shader_type: GLenum,
//     file: &str
// ) -> GLuint {
//     with_shader_file(file, |res| {
//         match res {
//             Err(err) => fail!("Failed opening shader file '{}':\r\n{}", file, err),
//             Ok(source) =>
//                 match compile_shader(shader_type, source) {
//                     Err(err) => fail!("Failed compiling shader file '{}':\r\n{}", file, err),
//                     Ok(shader) => shader
//                 },
//         }
//     })
// }

/// Reads a shader file and creates data required to compile.
// pub fn with_shader_file<T>(file: &str, f: |io::IoResult<&[&[u8]]>| -> T) -> T {
//     let path = path::Path::new(file);
//     let file = io::File::open(&path);
//     match file {
//         Err(err) => f(Err(err)),
//         Ok(file) => {
//             // Read the lines and tell errors if they occur.
//             let mut reader = io::BufferedReader::new(file);
//             let mut lines: Vec<~str> = Vec::new();
//             for line in reader.lines() {
//                 if line.is_err() {
//                     return f(Err(line.unwrap_err()));
//                 }

//                 lines.push(line.unwrap());
//             }

//             let lines: Vec<&[u8]> = lines.iter()
//                 .map(|line| line.as_bytes()).collect();
//             f(Ok(lines.as_slice()))
//         }
//     }
// }

/// Compiles a shader.
/// Returns a shader or a message with the error.
pub fn compile_shader(
    shader_type: GLenum,
    source: &str
) -> Result<GLuint, StrBuf> {
    let shader = gl::CreateShader(shader_type);
    unsafe {
        source.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
        gl::CompileShader(shader);
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status == (gl::TRUE as GLint) {
            Ok(shader)
        } else {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);

            gl::DeleteShader(shader);

            Err(StrBuf::from_utf8(buf).ok().expect("ShaderInfoLog not valid utf8"))
        }
    }
}


