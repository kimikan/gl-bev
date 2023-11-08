mod buffer;
mod program;
mod shader;
mod texture;
mod vertex_array;

pub use buffer::*;
pub use program::*;
pub use shader::*;
pub use texture::*;
pub use vertex_array::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
