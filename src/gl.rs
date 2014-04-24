//! OpenGL back-end for Rust-Graphics.

use gl = opengles::gl2;
use shader_utils::{with_shader_source, compile_shader};
use BackEnd = graphics::BackEnd;

static VERTEX_SHADER_TRI_LIST_XY_RGBA: &'static str = "
attribute vec4 a_v4Position;
attribute vec4 a_v4FillColor;
 
varying vec4 v_v4FillColor;
 
void main()
{
    v_v4FillColor = a_v4FillColor;
    gl_Position = a_v4Position;
}
";

static FRAGMENT_SHADER_TRI_LIST_XY_RGBA: &'static str = "
varying vec4 v_v4FillColor;
 
void main()
{
        gl_FragColor = v_v4FillColor;
}
";

/// OpenGL back-end for Rust-Graphics.
pub struct Gl {
    vertex_shader: gl::GLuint,
    fragment_shader: gl::GLuint,
    program: gl::GLuint,
    a_v4Position: gl::GLuint,
    a_v4FillColor: gl::GLuint,
    position_id: gl::GLuint,
    fill_color_id: gl::GLuint,
}


impl Gl {
    /// Creates a new Gl.
    pub fn new() -> Gl {
        let vertex_shader = with_shader_source(
            VERTEX_SHADER_TRI_LIST_XY_RGBA, |src| {
                compile_shader(gl::VERTEX_SHADER, src)
            }).unwrap();
        let fragment_shader = with_shader_source(
            FRAGMENT_SHADER_TRI_LIST_XY_RGBA, |src| {
                compile_shader(gl::FRAGMENT_SHADER, src)
            }).unwrap();
        let program = gl::create_program();
        gl::attach_shader(program, vertex_shader);
        gl::attach_shader(program, fragment_shader);
        gl::link_program(program);
        gl::use_program(program);
        let a_v4Position = 
            gl::get_attrib_location(program, "a_v4Position") as gl::GLuint;
        gl::enable_vertex_attrib_array(a_v4Position);
        let a_v4FillColor = 
            gl::get_attrib_location(program, "a_v4FillColor") as gl::GLuint;
        gl::enable_vertex_attrib_array(a_v4FillColor);

        // Load the vertices and color buffers.
        let buffers = gl::gen_buffers(2);
        let position_id = *buffers.get(0);
        let fill_color_id = *buffers.get(1);

        Gl {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
            a_v4Position: a_v4Position,
            a_v4FillColor: a_v4FillColor,
            position_id: position_id,
            fill_color_id: fill_color_id,
        }
    }
}

impl BackEnd for Gl {
    fn supports_clear_rgba(&self) -> bool { true }

    fn clear_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        gl::clear_color(r, g, b, a);
        gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    fn alpha_blend(&mut self, on: bool) {
        if on {
            gl::enable(gl::BLEND);
            gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        } else {
            gl::disable(gl::BLEND);
        }
    }

    fn supports_tri_list_xy_rgba_f32(&self) -> bool { true }

    fn tri_list_xy_rgba_f32(&mut self, vertices: &[f32], colors: &[f32]) {
        let size_vertices: i32 = 2;
        gl::bind_buffer(gl::ARRAY_BUFFER, self.position_id);
        gl::buffer_data(gl::ARRAY_BUFFER, vertices.as_slice(), gl::DYNAMIC_DRAW);
        gl::vertex_attrib_pointer_f32(self.a_v4Position, size_vertices, true, 0, 0);

        gl::bind_buffer(gl::ARRAY_BUFFER, self.fill_color_id);
        gl::buffer_data(gl::ARRAY_BUFFER, colors.as_slice(), gl::DYNAMIC_DRAW);
        gl::vertex_attrib_pointer_f32(self.a_v4FillColor, 4, false, 0, 0);

        // gl::enable(gl::DEPTH_TEST);
        gl::cull_face(gl::FRONT_AND_BACK);

        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::draw_arrays(gl::TRIANGLES, 0, items);
    }
}

