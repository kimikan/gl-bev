use gl::types::GLsizei;
use std::path::Path;
use std::ptr;

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
precision highp float;

in vec3 position;

uniform mat4 view;
uniform mat4 projection;
uniform mat4 model;
  
void main() {
    gl_Position =  projection * (view* (model  * vec4(position,1.0)));
    //gl_Position = vec4(normalize(gl_Position.xyz), 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
precision highp float;

out vec4 FragColor;
void main() {
    FragColor = vec4(1.0,0.0,0.0,1.0);
}
"#;

type Pos = [f32; 3];

#[repr(C, packed)]
#[derive(Debug)]
struct Vertex(Pos);

struct Face([i32; 3]);

fn build() -> (Vec<Vertex>, Vec<i32>) {
    let start_r = 0.4f32;
    let start_y = -0.5;
    let step_r = 0.02f32;
    let step_y = 0.05f32;

    let mut vertices = vec![];
    let mut indices = vec![];

    let mut n = 0;
    for i in 0..20 {
        for j in 0..73 {
            let y = start_y + step_y * (i as f32);
            let r = start_r + (i as f32) * step_r;

            let radian = (((j + 1) * 5) as f32).to_radians();
            let x = radian.sin() * r;
            let z = radian.cos() * r;

            vertices.push(Vertex([x, y, z]));

            if n >= 73 {
                indices.push(n - 72);
                indices.push(n - 73);
                indices.push(n - 1);
                indices.push(n);
                indices.push(n - 1);
                indices.push(n - 72);

                println!("{}  {}  {}", x, y, z);
            } // end if
            n += 1;
            println!("{}", n);
        }
    } //end for

    println!("indices: {}", indices.len());
    (vertices, indices)
}

pub struct Bowl {
    program: gl_wrapper::ShaderProgram,
    _vertex_buffer: gl_wrapper::Buffer,
    _index_buffer: gl_wrapper::Buffer,
    vertex_array: gl_wrapper::VertexArray,
}

impl Bowl {
    pub fn new() -> anyhow::Result<Self> {
        unsafe {
            let vertex_shader = gl_wrapper::Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader =
                gl_wrapper::Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = gl_wrapper::ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_array = gl_wrapper::VertexArray::new();
            vertex_array.bind();

            let mut vertex_buffer = gl_wrapper::Buffer::new(gl::ARRAY_BUFFER);
            let (vertices, indices) = build();
            vertex_buffer.set_data(&vertices[..], gl::STATIC_DRAW);

            let mut index_buffer = gl_wrapper::Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&indices[..], gl::STATIC_DRAW);

            let pos_attrib = program.get_attrib_location("position")?;
            gl_wrapper::set_attribute!(vertex_array, pos_attrib, Vertex::0);
            program.set_int_uniform("texture0", 0)?;

            //ssssssssssssssssssssssssssssssssssssss
            let camera = nalgebra_glm::vec3(0.0, 0.5, 0.5);
            let center = nalgebra_glm::vec3(0.0, 0.0, 0.0);
            let direction = nalgebra_glm::normalize(&(center - camera));
            let camera_right = nalgebra_glm::vec3(-1.0, 0.0, 0.0);
            let up = nalgebra_glm::normalize(&nalgebra_glm::cross(&direction, &camera_right));
            //let up = nalgebra_glm::vec3(0.0, 1.0, 0.0);
            let view = nalgebra_glm::look_at(&camera, &center, &up);
            println!("{}", view);
            //let projection = nalgebra_glm::perspective(1.0f32, 75f32.to_radians(), 0.05f32, 100f32);
            let projection = nalgebra_glm::ortho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);

            let mut model = nalgebra_glm::Mat4::new_scaling(0.4);
            program.set_mat4_uniformf("projection", &projection)?;
            program.set_mat4_uniformf("view", &view)?;
            program.set_mat4_uniformf("model", &model)?;
            //ssssssssssssssssssssssssssssssssssssssss

            //gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            //gl::Enable(gl::BLEND);
            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                _index_buffer: index_buffer,
                vertex_array,
            })
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.apply();
            self.vertex_array.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                self._index_buffer.element_count() as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}
