use gl::types::*;

pub struct Buffer {
    pub id: GLuint,
    target: GLuint,

    count: usize,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        Self {
            id,
            target,
            count: 0,
        }
    }

    pub fn element_count(&self) -> usize {
        self.count
    }

    pub unsafe fn set_data<D>(&mut self, data: &[D], usage: GLuint) {
        self.bind();
        self.count = data.len();
        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage,
        );
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }
}
