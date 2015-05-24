#[macro_use(gfx_vertex, gfx_parameters)]
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate carboxyl;
extern crate carboxyl_window;
extern crate window;
extern crate input;
extern crate shader_version;
extern crate glutin_window;
extern crate gfx_func;
extern crate cgmath;

use cgmath::FixedArray;
use std::rc::Rc;
use std::cell::RefCell;
use carboxyl::Signal;
use carboxyl_window::{ RunnableWindow, SourceWindow };
use window::{ WindowSettings };
use shader_version::OpenGL;
use glutin_window::GlutinWindow;
use gfx::traits::FactoryExt;
use gfx::{ Stream, ClearData };
use gfx::batch::{ OwnedBatch };
use gfx_func::element::{ Batch, Cleared, Draw };

pub mod shared_win;


gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
    a_Color@ color: [f32; 3],
});

gfx_parameters!( Params {
    model_view_proj@ model_view_proj: [[f32; 4]; 4],
});

fn main() {
    const GLVERSION: OpenGL = OpenGL::_2_1;
    let settings = WindowSettings::new("gfx + carboxyl_window", (640, 480));
    let window = Rc::new(RefCell::new(GlutinWindow::new(GLVERSION, settings)));
    let (mut stream, mut device, mut factory) = shared_win::init_shared(window.clone());
    let mut source = SourceWindow::new(window.clone());

    let element = {
        let vertex_data = [
            Vertex { pos: [ -0.5, -0.5, -1.0 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [  0.5, -0.5, -1.0 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5, -1.0 ], color: [0.0, 0.0, 1.0] },
        ];

        let data = Params {
            model_view_proj: cgmath::perspective(cgmath::deg(60.0f32),
                                      stream.get_aspect_ratio(),
                                      0.1, 1000.0
                                      ).into_fixed(),
            _r: std::marker::PhantomData,
        };

        let mesh = factory.create_mesh(&vertex_data);
        let program = {
            let vs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslv")),
                .. gfx::ShaderSource::empty()
            };
            let fs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslf")),
                .. gfx::ShaderSource::empty()
            };
            factory.link_program_source(vs, fs).unwrap()
        };
        Cleared::new(
            ClearData { color: [0.3, 0.3, 0.3, 1.0], depth: 1.0, stencil: 0 },
            Batch(OwnedBatch::new(mesh, program, data).unwrap()),
        )
    };
    let signal = Signal::new(element);

    source.run_with(120.0, || {
        let current = signal.sample();
        current.draw(&mut stream);
        stream.present(&mut device);
    });
}
