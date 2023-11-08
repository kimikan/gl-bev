use crate::shader::{Shader, ShaderError};
use gl::types::*;
use std::ffi::CString;

pub struct ShaderProgram {
    pub id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, ShaderError> {
        let attrib = CString::new(attrib)?;
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }

    pub unsafe fn set_int_uniform(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        self.apply();
        let uniform = CString::new(name)?;
        gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);
        Ok(())
    }

    pub unsafe fn set_mat4_uniformf(
        &self,
        name: &str,
        value: &nalgebra_glm::TMat4<f32>,
    ) -> Result<(), ShaderError> {
        self.apply();
        let uniform = CString::new(name)?;
        let location = gl::GetUniformLocation(self.id, uniform.as_ptr());
        let e1 = gl::GetError();
        gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());

        let e2 = gl::GetError();
        println!("value: {}, err: {}", value, e1 + e2);
        Ok(())
    }

    pub unsafe fn set_mat4_uniformd(
        &self,
        name: &str,
        value: &nalgebra_glm::TMat4<f64>,
    ) -> Result<(), ShaderError> {
        self.apply();
        let uniform = CString::new(name)?;
        gl::UniformMatrix4dv(
            gl::GetUniformLocation(self.id, uniform.as_ptr()),
            1,
            gl::FALSE,
            value.as_ptr(),
        );
        let e = gl::GetError();
        println!("value: {}, err: {}", value, e);
        Ok(())
    }
}
