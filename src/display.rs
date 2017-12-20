use glium;
use glium::{glutin, Surface};

pub fn init_display(events_loop: &mut glutin::EventsLoop) -> glium::Display {
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    display
}


pub fn create_program(display: &glium::Display) -> glium::Program {
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

    glium::Program::from_source(display,
                                vertex_shader_src,
                                fragment_shader_src,
                                None).unwrap()
}

pub fn draw(display: &glium::Display,
            program: &glium::Program,
            image_vec: Vec<u8>,
            dimensions: (u32, u32)) {
    let image = glium::texture::RawImage2d::from_raw_rgba(image_vec, dimensions);
    let texture = glium::texture::Texture2d::new(display, image).unwrap();
    let mag_texture = texture.sampled()
                          .magnify_filter(
                              glium::uniforms::MagnifySamplerFilter::Nearest);

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

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);


    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let uniforms = uniform! {
        tex: mag_texture,
    };

    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();
}
