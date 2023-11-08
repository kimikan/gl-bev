use std::path::Path;
use std::ptr;

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
precision highp float;

in vec3 position;
in vec2 vertexTexCoord;

out vec2 texCoord;

uniform mat4 projection;
uniform mat4 view;

void main() {
    gl_Position =  projection * vec4(position,1.0);
    gl_Position = vec4(normalize(gl_Position.xyz), 1.0);
    texCoord = vertexTexCoord;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
precision highp float;

out vec4 FragColor;
in vec2 texCoord;

uniform sampler2D texture0;

void main() {
    vec4 color0 = texture(texture0, texCoord);
    FragColor = mix(color0, vec4(1.0,0.0,0.0,1.0), 0.5);
    //FragColor = vec4(1.0,0.0,0.0,1.0);
}
"#;

type Pos = [f32; 3];
type TextureCoords = [f32; 2];

#[repr(C, packed)]
struct Vertex(Pos, TextureCoords);

//#[rustfmt::skip]
const VERTICES: [Vertex; 8] = [
    Vertex([0.0, -0.2, 0.5], [0.0, 0.0]), //0
    Vertex([0.5, 0.0, 0.0], [0.3, 0.0]),  //1
    Vertex([0.0, 0.2, -0.5], [0.3, 0.3]), //2
    Vertex([-0.5, 0.0, 0.0], [0.0, 0.3]), //3
    Vertex([0.0, 0.3, 0.5], [0.0, 0.6]),  //4
    Vertex([0.5, 0.5, 0.0], [0.6, 0.3]),  //5
    Vertex([0.0, 0.7, -0.5], [0.3, 0.3]), //6
    Vertex([-0.5, 0.5, 0.0], [0.3, 0.0]), //7
];

struct Face([i32; 6]);

//#[rustfmt::skip]
const FACES: &[Face] = &[
    Face([0, 1, 2, 2, 3, 0]), //bottom
    Face([0, 3, 7, 7, 4, 0]), //left-1
    Face([0, 4, 5, 5, 1, 0]), //right-1
    Face([1, 5, 6, 6, 2, 1]), //righ-2
    Face([6, 2, 3, 3, 7, 6]), //left-2
    Face([4, 5, 6, 6, 7, 4]), //top
];

fn get_indices() -> Vec<i32> {
    let ret = FACES.into_iter().map(|v| v.0).collect::<Vec<_>>();
    let z = ret.into_iter().flatten().collect();

    println!("{:?}", z);
    z
}

pub struct Cube {
    program: gl_wrapper::ShaderProgram,
    _vertex_buffer: gl_wrapper::Buffer,
    _index_buffer: gl_wrapper::Buffer,
    vertex_array: gl_wrapper::VertexArray,
    texture0: gl_wrapper::Texture,
}

impl Cube {
    pub fn new() -> anyhow::Result<Self> {
        unsafe {
            let vertex_shader = gl_wrapper::Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader =
                gl_wrapper::Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = gl_wrapper::ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_array = gl_wrapper::VertexArray::new();
            vertex_array.bind();

            let mut vertex_buffer = gl_wrapper::Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);

            let mut index_buffer = gl_wrapper::Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            let indices = get_indices();
            index_buffer.set_data(&indices[..], gl::STATIC_DRAW);

            let pos_attrib = program.get_attrib_location("position")?;
            gl_wrapper::set_attribute!(vertex_array, pos_attrib, Vertex::0);
            let color_attrib = program.get_attrib_location("vertexTexCoord")?;
            gl_wrapper::set_attribute!(vertex_array, color_attrib, Vertex::1);

            let texture0 = gl_wrapper::Texture::new();
            texture0.set_wrapping(gl::REPEAT);
            texture0.set_filtering(gl::LINEAR);
            texture0.load(&Path::new("assets/J5.png"))?;
            program.set_int_uniform("texture0", 0)?;

            //ssssssssssssssssssssssssssssssssssssss
            let camera = nalgebra_glm::vec3(0.0, 10.0, 0.0);
            let target = nalgebra_glm::vec3(0.0, 0.0, 0.0);
            let direction = nalgebra_glm::normalize(&(target - camera));
            let camera_right = nalgebra_glm::vec3(1.0, 0.0, 0.0);
            let up = nalgebra_glm::normalize(&nalgebra_glm::cross(&direction, &camera_right));
            //let up = nalgebra_glm::vec3(0.0, 1.0, 0.0);
            let view = nalgebra_glm::look_at(&camera, &target, &up);
            println!("{}", view);
            let projection =
                nalgebra_glm::perspective(1.0f32, std::f32::consts::PI / 4.0, 0.1f32, 100.0f32);
            program.set_mat4_uniformf("projection", &projection);
            program.set_mat4_uniformf("view", &view);
            //ssssssssssssssssssssssssssssssssssssssss

            //gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            //gl::Enable(gl::BLEND);
            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                _index_buffer: index_buffer,
                vertex_array,
                texture0,
            })
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.texture0.activate(gl::TEXTURE0);
            self.program.apply();
            self.vertex_array.bind();
            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
