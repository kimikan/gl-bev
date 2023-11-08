mod bowl;
mod cube;
mod mesh;

use crate::bowl::Bowl;
use gl::types::GLint;
use glfw::{Action, Context, Key};
use std::ffi::c_void;
use std::fs;
use std::ops::Mul;
use std::sync::mpsc::Receiver;

const SRC_WIDTH: u32 = 1024;
const SRC_HEIGHT: u32 = 768;

fn ui() -> anyhow::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(SRC_WIDTH, SRC_HEIGHT, "BOWL", glfw::WindowMode::Windowed)
        .expect("can NOT create window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut bowl = bowl::Bowl::new()?;
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            bowl.draw();
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    Ok(())
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => println!("YOU PRESSED A!"),
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    let camera = nalgebra_glm::vec3(0.0, 10.0, 0.0);
    let target = nalgebra_glm::vec3(0.0, 0.0, 0.0);

    let direction = nalgebra_glm::normalize(&(target - camera));
    let camera_right = nalgebra_glm::vec3(1.0, 0.0, 0.0);

    let up = nalgebra_glm::normalize(&nalgebra_glm::cross(&direction, &camera_right));
    //let up = nalgebra_glm::vec3(0.0, 1.0, 0.0);
    let view = nalgebra_glm::look_at(&camera, &target, &up);
    println!("{}", view);

    let projection = nalgebra_glm::perspective(1.0f32, std::f32::consts::PI / 4.0, 0.1f32, 20.0f32);
    println!("{}", projection);
    ui()
}
