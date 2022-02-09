use crate::special::chunk::Chunk;

pub trait Update {
    fn update(&mut self, _chunk: &mut Chunk) {}
    fn draw(&self) {}
}
