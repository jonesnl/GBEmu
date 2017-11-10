#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate glium;
extern crate image;

mod hw;
mod cpu;
mod registers;

use std::env;
use hw::controller::MBC1;
use hw::memory::Bus;
use hw::memory::Memory;
use cpu::Cpu;

use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

use std::io::Cursor;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("./opengl.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [ -1.0, -1.0 ], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ -1.0,  1.0 ], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [  1.0, -1.0 ], tex_coords: [1.0, 0.0] };
	let vertex4 = Vertex { position: [  1.0,  1.0 ], tex_coords: [1.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            tex: &texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }

    /*
    if std::env::args().len() != 2 {
        println!("Argument count is not 2!");
        std::process::exit(1);
    }

    let filename = env::args().nth(1).unwrap();

    let path = Path::new(&filename);

    let mut file = File::open(&path).unwrap();

    let mut rom = Vec::new();
    
    match file.read_to_end(&mut rom) {
        Ok(_) => (),
        Err(m) => {
            println!("Error loading game: {}", m);
            return;
        }
    }
    // From this point on the rom should never be modified
    let rom = rom;

    let new_cartridge: Box<Bus> = MBC1::new(rom);
    let new_memory = Memory::new(new_cartridge);
    let cpu = Cpu::new(new_memory);

    for x in 0x100..0x130 {
        if (x % 0x10) == 0 {println!("");}
        print!("{:0>2x} ", cpu.read8(x));
    }
    */
}
