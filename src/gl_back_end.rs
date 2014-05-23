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
use BackEnd = graphics::BackEnd;

// Local crate.
use AssetStore = asset_store::AssetStore;

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

/// OpenGL back-end for Rust-Graphics.
pub struct Gl<'a> {
    gl_data: &'a mut GlData,
    asset_store: &'a AssetStore,
}

impl<'a> Gl<'a> {
    /// Creates a new OpenGl back-end.
    pub fn new(
        gl_data: &'a mut GlData,
        asset_store: &'a AssetStore
    ) -> Gl<'a> {
        Gl {
            gl_data: gl_data,
            asset_store: asset_store,
        }
    }
}

struct TriListXYRGBA {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    a_v4Position: GLuint,
    a_v4FillColor: GLuint,
}

impl TriListXYRGBA {
    fn new() -> TriListXYRGBA {
        let vertex_shader = match compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER_TRI_LIST_XY_RGBA) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_TRI_LIST_XY_RGBA) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::UseProgram(program);
        unsafe {
            let a_v4Position = "a_v4Position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
            gl::EnableVertexAttribArray(a_v4Position as GLuint);
            let a_v4FillColor = "a_v4FillColor".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
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

impl TriListXYRGBAUV {
    fn new() -> TriListXYRGBAUV {
        let vertex_shader = match compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER_TRI_LIST_XY_RGBA_UV) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_TRI_LIST_XY_RGBA_UV) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::UseProgram(program);
        unsafe {
            let a_v4Position = "a_v4Position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
            gl::EnableVertexAttribArray(a_v4Position as GLuint);
            let a_v4FillColor = "a_v4FillColor".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
            gl::EnableVertexAttribArray(a_v4FillColor as GLuint);
            let a_v2TexCoord = "a_v2TexCoord".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
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
pub struct GlData {
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


impl<'a> GlData {
    /// Creates a new OpenGl back-end.
    pub fn new() -> GlData {
        // Load the vertices, color and texture coord buffers.
        unsafe {
            let mut vbo : [GLuint, ..3] = [0, ..3];
            gl::GenBuffers(3, vbo.as_mut_ptr());
            let position_id = vbo[0];
            let fill_color_id = vbo[1];
            let tex_coord_id = vbo[2];

            GlData {
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

impl<'a> BackEnd for Gl<'a> {
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

    fn enable_single_texture(&mut self, texture_id: uint) {
        let texture = self.asset_store.get_texture(texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    fn disable_single_texture(&mut self) {}

    // Assume all textures has alpha channel for now.
    fn has_texture_alpha(&self, _texture_id: uint) -> bool { true }

    fn supports_tri_list_xy_f32_rgba_f32(&self) -> bool { true }

    fn tri_list_xy_f32_rgba_f32(
        &mut self,
        vertices: &[f32],
        colors: &[f32]
    ) {
        {
            // Set shader program.
            let shader_program = self.gl_data.tri_list_xy_rgba.program;
            self.gl_data.use_program(shader_program);
        }
        let data = &self.gl_data;
        let shader = &data.tri_list_xy_rgba;
        let size_vertices: i32 = 2;
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER, data.position_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&vertices[0]), gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(
                shader.a_v4Position as GLuint, size_vertices, gl::FLOAT, gl::TRUE, 0, ptr::null());

            gl::BindBuffer(
                gl::ARRAY_BUFFER, data.fill_color_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, (colors.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&colors[0]), gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(
                shader.a_v4FillColor as GLuint, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
        }
        // gl::enable(gl::DEPTH_TEST);
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
            let shader_program = self.gl_data.tri_list_xy_rgba_uv.program;
            self.gl_data.use_program(shader_program);
        }
        let data = &self.gl_data;
        let shader = &data.tri_list_xy_rgba_uv;
        let size_vertices: i32 = 2;
        unsafe {
            gl::BindBuffer(
                gl::ARRAY_BUFFER, data.position_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&vertices[0]), gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(
                shader.a_v4Position as GLuint, size_vertices, gl::FLOAT, gl::TRUE, 0, ptr::null());

            gl::BindBuffer(
                gl::ARRAY_BUFFER, data.fill_color_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, (colors.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&colors[0]), gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(
                shader.a_v4FillColor as GLuint, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());

            gl::BindBuffer(
                gl::ARRAY_BUFFER, data.tex_coord_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, (texture_coords.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&texture_coords[0]), gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(
                shader.a_v2TexCoord as GLuint, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        }
        // gl::enable(gl::DEPTH_TEST);
        gl::CullFace(gl::FRONT_AND_BACK);

        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::DrawArrays(gl::TRIANGLES, 0, items);
    }
}

