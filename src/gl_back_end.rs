//! OpenGL back-end for Rust-Graphics.

// External crates.
use gl;
use gl::types::{
    GLfloat,
    GLsizeiptr,
    GLuint,
};
use std::ptr;
use std::mem;
use shader_utils::{compile_shader};
use graphics::BackEnd;

// Local crate.
use Texture;

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

static VERTEX_SHADER_TRI_LIST_XY_RGBA_UV: &'static str = "
attribute vec4 a_v4Position;
attribute vec4 a_v4FillColor;
attribute vec2 a_v2TexCoord;

uniform sampler2D s_texture;

varying vec2 v_v2TexCoord;
varying vec4 v_v4FillColor;

void main()
{
        v_v2TexCoord = a_v2TexCoord;
        v_v4FillColor = a_v4FillColor;
        gl_Position = a_v4Position;
}
";

static FRAGMENT_SHADER_TRI_LIST_XY_RGBA_UV: &'static str = "
uniform sampler2D s_texture;

varying vec2 v_v2TexCoord;
varying vec4 v_v4FillColor;

void main()
{
        gl_FragColor = texture2D(s_texture, v_v2TexCoord) * v_v4FillColor;
}
";

struct TriListXYRGBA {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    a_v4Position: GLuint,
    a_v4FillColor: GLuint,
}

impl Drop for TriListXYRGBA {
    fn drop(&mut self) {
        gl::DeleteProgram(self.program);
        gl::DeleteShader(self.vertex_shader);
        gl::DeleteShader(self.fragment_shader);
    }
}

impl TriListXYRGBA {
    fn new() -> TriListXYRGBA {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            VERTEX_SHADER_TRI_LIST_XY_RGBA      // shader source
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            FRAGMENT_SHADER_TRI_LIST_XY_RGBA    // shader source
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::UseProgram(program);
        unsafe {
            let a_v4Position = "a_v4Position".with_c_str(
                |ptr| gl::GetAttribLocation(program, ptr)
            );
            gl::EnableVertexAttribArray(a_v4Position as GLuint);
            let a_v4FillColor = "a_v4FillColor".with_c_str(
                |ptr| gl::GetAttribLocation(program, ptr)
            );
            gl::EnableVertexAttribArray(a_v4FillColor as GLuint);
            TriListXYRGBA {
                vertex_shader: vertex_shader,
                fragment_shader: fragment_shader,
                program: program,
                a_v4Position: a_v4Position as GLuint,
                a_v4FillColor: a_v4FillColor as GLuint,
            }
        }
    }
}

struct TriListXYRGBAUV {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    a_v4Position: GLuint,
    a_v4FillColor: GLuint,
    a_v2TexCoord: GLuint,
}

impl Drop for TriListXYRGBAUV {
    fn drop(&mut self) {
        gl::DeleteProgram(self.program);
        gl::DeleteShader(self.vertex_shader);
        gl::DeleteShader(self.fragment_shader);
    }
}

impl TriListXYRGBAUV {
    fn new() -> TriListXYRGBAUV {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            VERTEX_SHADER_TRI_LIST_XY_RGBA_UV   // shader type
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            FRAGMENT_SHADER_TRI_LIST_XY_RGBA_UV // shader source
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::UseProgram(program);
        unsafe {
            let a_v4Position = "a_v4Position".with_c_str(
                |ptr| gl::GetAttribLocation(program, ptr)
            );
            gl::EnableVertexAttribArray(a_v4Position as GLuint);
            let a_v4FillColor = "a_v4FillColor".with_c_str(
                |ptr| gl::GetAttribLocation(program, ptr)
            );
            gl::EnableVertexAttribArray(a_v4FillColor as GLuint);
            let a_v2TexCoord = "a_v2TexCoord".with_c_str(
                |ptr| gl::GetAttribLocation(program, ptr)
            );
            gl::EnableVertexAttribArray(a_v4FillColor as GLuint);
            TriListXYRGBAUV {
                vertex_shader: vertex_shader,
                fragment_shader: fragment_shader,
                program: program,
                a_v4Position: a_v4Position as GLuint,
                a_v4FillColor: a_v4FillColor as GLuint,
                a_v2TexCoord: a_v2TexCoord as GLuint,
            }
        }
    }
}

/// Contains OpenGL data.
pub struct Gl {
    tri_list_xy_rgba: TriListXYRGBA,
    tri_list_xy_rgba_uv: TriListXYRGBAUV,
    // id of buffer for xy positions.
    position_id: GLuint,
    // id of buffer for rgba colors.
    fill_color_id: GLuint,
    // id of buffer for uv texture coords.
    tex_coord_id: GLuint,
    // Keeps track of the current shader program.
    current_program: Option<GLuint>,
}


impl<'a> Gl {
    /// Creates a new OpenGl back-end.
    pub fn new() -> Gl {
        // Load the vertices, color and texture coord buffers.
        unsafe {
            let mut vbo : [GLuint, ..3] = [0, ..3];
            gl::GenBuffers(3, vbo.as_mut_ptr());
            let position_id = vbo[0];
            let fill_color_id = vbo[1];
            let tex_coord_id = vbo[2];

            Gl {
                tri_list_xy_rgba: TriListXYRGBA::new(),
                tri_list_xy_rgba_uv: TriListXYRGBAUV::new(),
                position_id: position_id,
                fill_color_id: fill_color_id,
                tex_coord_id: tex_coord_id,
                current_program: None,
            }
        }
    }

    /// Sets the current program only if the program is not in use.
    pub fn use_program(&mut self, program: GLuint) {
        match self.current_program {
            None => {},
            Some(current_program) => {
                if program == current_program { return; }
            },
        }

        gl::UseProgram(program);
        self.current_program = Some(program);
    }
}

impl BackEnd<Texture> for Gl {
    fn supports_clear_rgba(&self) -> bool { true }

    fn clear_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    fn enable_alpha_blend(&mut self) {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    fn disable_alpha_blend(&mut self) {
        gl::Disable(gl::BLEND);
    }

    fn supports_single_texture(&self) -> bool { true }

    fn enable_single_texture(&mut self, texture: &Texture) {
        let texture = texture.get_id();
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    fn disable_single_texture(&mut self) {}

    // Assume all textures has alpha channel for now.
    fn has_texture_alpha(&self, _texture: &Texture) -> bool { true }

    fn supports_tri_list_xy_f32_rgba_f32(&self) -> bool { true }

    fn tri_list_xy_f32_rgba_f32(
        &mut self,
        vertices: &[f32],
        colors: &[f32]
    ) {
        {
            // Set shader program.
            let shader_program = self.tri_list_xy_rgba.program;
            self.use_program(shader_program);
        }
        let shader = &self.tri_list_xy_rgba;
        // xy makes two floats.
        let size_vertices: i32 = 2;
        let vertices_byte_len = (
                vertices.len() * mem::size_of::<GLfloat>()
            ) as GLsizeiptr;
        let normalize_vertices = gl::FALSE;
        // The data is tightly packed.
        let stride_vertices = 0;
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER,   // buffer type
                self.position_id    // position buffer for xy coordinates
            );
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                vertices_byte_len, 
                mem::transmute(&vertices[0]), 
                gl::DYNAMIC_DRAW
            );
            gl::VertexAttribPointer(
                shader.a_v4Position as GLuint, 
                size_vertices,
                gl::FLOAT,
                normalize_vertices,
                stride_vertices, 
                ptr::null()
            );
        }

        // rgba makes 4 floats.
        let size_fill_colors = 4;
        let normalize_colors = gl::FALSE;
        let fill_colors_byte_len = (
                colors.len() * mem::size_of::<GLfloat>()
            ) as GLsizeiptr;
        // The data is tightly packed.
        let stride_fill_colors = 0;
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER, 
                self.fill_color_id
            );
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                fill_colors_byte_len, 
                mem::transmute(&colors[0]), 
                gl::DYNAMIC_DRAW
            );
            gl::VertexAttribPointer(
                shader.a_v4FillColor as GLuint, 
                size_fill_colors, 
                gl::FLOAT, 
                normalize_colors, 
                stride_fill_colors, 
                ptr::null()
            );
        }
        
        // Render triangles whether they are facing 
        // clockwise or counter clockwise.
        gl::CullFace(gl::FRONT_AND_BACK);

        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::DrawArrays(gl::TRIANGLES, 0, items);
    }

    fn supports_tri_list_xy_f32_rgba_f32_uv_f32(&self) -> bool { true }

    fn tri_list_xy_f32_rgba_f32_uv_f32(
        &mut self,
        vertices: &[f32],
        colors: &[f32],
        texture_coords: &[f32]
    ) {
        {
            // Set shader program.
            let shader_program = self.tri_list_xy_rgba_uv.program;
            self.use_program(shader_program);
        }
        let shader = &self.tri_list_xy_rgba_uv;
        let size_vertices: i32 = 2;
        let normalize_vertices = gl::FALSE;
        let vertices_byte_len = (
                vertices.len() * mem::size_of::<GLfloat>()
            ) as GLsizeiptr;
        // The data is tightly packed.
        let stride_vertices = 0;
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER, self.position_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                vertices_byte_len, 
                mem::transmute(&vertices[0]), 
                gl::DYNAMIC_DRAW
            );
            gl::VertexAttribPointer(
                shader.a_v4Position as GLuint, 
                size_vertices, 
                gl::FLOAT, 
                normalize_vertices,
                stride_vertices, 
                ptr::null()
            );
        }

        // rgba makes 4 floats.
        let size_fill_color = 4;
        let normalize_fill_color = gl::FALSE;
        let fill_colors_byte_len = (
                colors.len() * mem::size_of::<GLfloat>()
            ) as GLsizeiptr;
        // The data is tightly packed.
        let stride_fill_colors = 0;
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER, 
                self.fill_color_id
            );
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                fill_colors_byte_len, 
                mem::transmute(&colors[0]), 
                gl::DYNAMIC_DRAW
            );
            gl::VertexAttribPointer(
                shader.a_v4FillColor as GLuint, 
                size_fill_color, 
                gl::FLOAT, 
                normalize_fill_color,
                stride_fill_colors, 
                ptr::null()
            );
        }

        // uv makes two floats.
        let size_tex_coord = 2;
        let texture_coords_byte_len = (
                texture_coords.len() * mem::size_of::<GLfloat>()
            ) as GLsizeiptr;
        let normalize_texture_coords = gl::FALSE;
        // The data is tightly packed.
        let stride_texture_coords = 0;
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER, 
                self.tex_coord_id
            );
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                texture_coords_byte_len, 
                mem::transmute(&texture_coords[0]), 
                gl::DYNAMIC_DRAW
            );
            gl::VertexAttribPointer(
                shader.a_v2TexCoord as GLuint, 
                size_tex_coord, 
                gl::FLOAT, 
                normalize_texture_coords,
                stride_texture_coords, 
                ptr::null()
            );
        }
        
        // Render triangles whether they are facing 
        // clockwise or counter clockwise.
        gl::CullFace(gl::FRONT_AND_BACK);

        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::DrawArrays(gl::TRIANGLES, 0, items);
    }
}

