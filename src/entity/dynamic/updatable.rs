use crate::special::chunk::Chunk;

pub trait Update {
    fn update(&mut self, chunk: &mut Chunk) {}
    fn draw(&self) {}
}
