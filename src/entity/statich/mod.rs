pub mod road;
pub mod stone;
pub mod terrain;

#[derive(Debug)]
pub enum Static {
    Stone(stone::Stone),
    Road(road::Segment),
    Terrain(terrain::Terrain),
}
