#![feature(globs)]

extern crate piston;

extern crate hgl;
extern crate gl;
extern crate sdl2_game_window;

use sdl2_game_window::GameWindowSDL2;
use piston::{
    Game, 
    GameIteratorSettings,
    GameWindowSettings, 
    RenderArgs
};

use std::mem::size_of;
use hgl::{Shader, Program, Triangles, Vbo, Vao};

#[allow(dead_code)]
pub struct App {
    program: Program,
    vao: Vao,
    vbo: Vbo
}

static VERTEX_SHADER: &'static str = r"
    attribute vec2 position;
    
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
";

static FRAGMENT_SHADER: &'static str = r"
    void main() {
        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    }
";

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        let vao = Vao::new();
        vao.bind();

        let program = Program::link([Shader::compile(VERTEX_SHADER, hgl::VertexShader),
        Shader::compile(FRAGMENT_SHADER, hgl::FragmentShader)]).unwrap();
        program.bind();

        let vbo = Vbo::from_data([
            0.0f32, 0.5, 1.0, 0.0, 0.0,
            0.5,   -0.5, 0.0, 1.0, 0.0,
            -0.5,  -0.5, 0.0, 0.0, 1.0
        ], hgl::StaticDraw);

        vao.enable_attrib(&program, "position", gl::FLOAT, 2, 5*size_of::<f32>() as i32, 0);
        vao.enable_attrib(&program, "color", gl::FLOAT, 3, 5*size_of::<f32>() as i32, 2*size_of::<f32>());
        vbo.bind();

        App {
            program: program,
            vao: vao,
            vbo: vbo
        }
    }
}

impl Game for App {
    fn render(&mut self, args: &mut RenderArgs) {
        gl::Viewport(0, 0, args.width as i32, args.height as i32);
        gl::ClearColor(0.0, 0.0, 0.0, 0.1);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        self.vao.draw_array(Triangles, 0, 3);
    }
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Test".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    App::new().run(&mut window, &game_iter_settings);
}

